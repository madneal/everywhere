use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tantivy::collector::TopDocs;
use tantivy::query::{Query, QueryParser};
use tantivy::schema::*;
use tantivy::snippet::SnippetGenerator;
use tantivy::tokenizer::{
    LowerCaser, RegexTokenizer, RemoveLongFilter, TextAnalyzer,
};
use tantivy::{Index, IndexReader, IndexWriter, Searcher, TantivyDocument};

const MIXED_TOKENIZER: &str = "mixed_cjk";
const ROOTS_JSON: &str = "indexed_source_roots.json";
const TOKEN_PATTERN: &str = r"[A-Za-z0-9_]+|[\u{4E00}-\u{9FFF}\u{3400}-\u{4DBF}\u{3040}-\u{309F}\u{30A0}-\u{30FF}]";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub snippet_highlights: Vec<(usize, usize)>,
    pub path: PathBuf,
    pub score: f32,
}

pub struct SearchEngine {
    index: Index,
    reader: IndexReader,
    query_parser: QueryParser,
    title_field: Field,
    content_field: Field,
    path_field: Field,
}

fn build_schema() -> Schema {
    let text_indexing = TextFieldIndexing::default()
        .set_tokenizer(MIXED_TOKENIZER)
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_indexing)
        .set_stored();
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", text_options.clone());
    schema_builder.add_text_field("content", text_options);
    schema_builder.add_text_field("path", STRING | STORED);
    schema_builder.build()
}

fn register_mixed_tokenizer(index: &Index) -> Result<()> {
    let rx = RegexTokenizer::new(TOKEN_PATTERN).map_err(|e| anyhow::anyhow!("{}", e))?;
    let analyzer = TextAnalyzer::builder(rx)
        .filter(RemoveLongFilter::limit(200))
        .filter(LowerCaser)
        .build();
    index.tokenizers().register(MIXED_TOKENIZER, analyzer);
    Ok(())
}

fn content_uses_mixed_tokenizer(schema: &Schema) -> bool {
    let Ok(f) = schema.get_field("content") else {
        return false;
    };
    let FieldType::Str(opts) = schema.get_field_entry(f).field_type() else {
        return false;
    };
    opts.get_indexing_options()
        .is_some_and(|io| io.tokenizer() == MIXED_TOKENIZER)
}

fn wipe_index_preserve_roots(index_path: &Path) -> Result<()> {
    let roots = std::fs::read(index_path.join(ROOTS_JSON)).ok();
    if index_path.exists() {
        std::fs::remove_dir_all(index_path)?;
    }
    std::fs::create_dir_all(index_path)?;
    if let Some(bytes) = roots {
        std::fs::write(index_path.join(ROOTS_JSON), bytes)?;
    }
    Ok(())
}

fn join_stored_field(doc: &TantivyDocument, field: Field) -> String {
    use tantivy::schema::document::Value;
    doc.get_all(field)
        .filter_map(|v| {
            v.as_str()
                .map(str::to_string)
                .or_else(|| v.as_pre_tokenized_text().map(|p| p.text.clone()))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_subslice_chars(hay: &[char], needle: &[char]) -> Option<usize> {
    if needle.is_empty() || hay.len() < needle.len() {
        return None;
    }
    hay.windows(needle.len()).position(|w| w == needle)
}

fn query_match_char_span(full_chars: &[char], query_str: &str) -> Option<(usize, usize)> {
    let q = query_str.trim();
    if q.is_empty() || full_chars.is_empty() {
        return None;
    }
    let qchars: Vec<char> = q.chars().collect();
    if let Some(i) = find_subslice_chars(full_chars, &qchars) {
        return Some((i, i + qchars.len()));
    }
    let mut spans: Vec<(usize, usize)> = Vec::new();
    for t in q.split_whitespace() {
        let tc: Vec<char> = t.chars().collect();
        if tc.is_empty() {
            continue;
        }
        if let Some(i) = find_subslice_chars(full_chars, &tc) {
            spans.push((i, i + tc.len()));
        }
    }
    if spans.is_empty() {
        None
    } else {
        let a = spans.iter().map(|(s, _)| *s).min().unwrap();
        let b = spans.iter().map(|(_, e)| *e).max().unwrap();
        Some((a, b))
    }
}

fn query_match_char_span_with_byte_fallback(full: &str, query_str: &str) -> Option<(usize, usize)> {
    let full_chars: Vec<char> = full.chars().collect();
    if let Some(sp) = query_match_char_span(&full_chars, query_str) {
        return Some(sp);
    }
    let q = query_str.trim();
    if q.is_empty() {
        return None;
    }
    full.find(q).map(|byte_i| {
        let start_c = full[..byte_i].chars().count();
        let end_c = start_c + q.chars().count();
        (start_c, end_c)
    })
}

fn byte_at_char_idx(s: &str, char_idx: usize) -> usize {
    if char_idx == 0 {
        return 0;
    }
    s.char_indices()
        .nth(char_idx)
        .map(|(b, _)| b)
        .unwrap_or(s.len())
}

const SNIPPET_SOURCE_MAX_CHARS: usize = 48_000;
const SNIPPET_OUT_MAX_CHARS: usize = 4_000;
const DISK_SNIPPET_MAX_BYTES: usize = 256 * 1024;

fn trim_chars_clone(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        s.chars().take(max_chars).collect()
    }
}

fn finalize_snippet(text: String, hl: Vec<(usize, usize)>) -> (String, Vec<(usize, usize)>) {
    if text.chars().count() <= SNIPPET_OUT_MAX_CHARS {
        (text, hl)
    } else {
        (trim_chars_clone(&text, SNIPPET_OUT_MAX_CHARS), Vec::new())
    }
}

fn read_path_prefix_utf8(path: &Path, max_bytes: usize) -> Option<String> {
    use std::io::Read;
    let mut file = std::fs::File::open(path).ok()?;
    let mut buf = vec![0u8; max_bytes];
    let n = file.read(&mut buf).ok()?;
    buf.truncate(n);
    Some(String::from_utf8_lossy(&buf).into_owned())
}

fn manual_excerpt(full: &str, query_str: &str, margin_chars: usize, max_chars: usize) -> (String, Vec<(usize, usize)>) {
    let full_chars: Vec<char> = full.chars().collect();
    if full_chars.is_empty() {
        return (String::new(), Vec::new());
    }
    let Some((mc, me)) = query_match_char_span_with_byte_fallback(full, query_str) else {
        let take = max_chars.min(full_chars.len());
        let s: String = full_chars[..take].iter().collect();
        return (s, Vec::new());
    };
    let mut from_c = mc.saturating_sub(margin_chars);
    let mut to_c = (me + margin_chars).min(full_chars.len());
    if to_c - from_c > max_chars {
        let half = max_chars / 2;
        from_c = mc.saturating_sub(half);
        to_c = (from_c + max_chars).min(full_chars.len());
        if to_c - from_c < max_chars {
            from_c = to_c.saturating_sub(max_chars);
        }
    }
    let excerpt: String = full_chars[from_c..to_c].iter().collect();
    let hs = mc - from_c;
    let he = me - from_c;
    let start_b = byte_at_char_idx(&excerpt, hs);
    let end_b = byte_at_char_idx(&excerpt, he);
    (excerpt, vec![(start_b, end_b)])
}

fn load_disk_plain(path: &Path) -> Option<String> {
    if !path.exists() || path.is_dir() {
        return None;
    }
    let raw = read_path_prefix_utf8(path, DISK_SNIPPET_MAX_BYTES)?;
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    let body = match ext.as_str() {
        "md" | "mdx" | "markdown" => crate::parser::markdown_plain_from_bytes(&raw),
        _ => raw,
    };
    if body.trim().is_empty() {
        None
    } else {
        Some(body)
    }
}

impl SearchEngine {
    pub fn new(index_path: &str) -> Result<Self> {
        let index_path = Path::new(index_path);
        let meta = index_path.join("meta.json");

        let index = if meta.exists() {
            let idx = Index::open_in_dir(index_path)?;
            if content_uses_mixed_tokenizer(&idx.schema()) {
                idx
            } else {
                wipe_index_preserve_roots(index_path)?;
                Index::create_in_dir(index_path, build_schema())?
            }
        } else {
            wipe_index_preserve_roots(index_path)?;
            Index::create_in_dir(index_path, build_schema())?
        };

        register_mixed_tokenizer(&index)?;

        let reader = index.reader()?;
        let schema = index.schema();
        let title_field = schema.get_field("title")?;
        let content_field = schema.get_field("content")?;
        let path_field = schema.get_field("path")?;

        let mut query_parser = QueryParser::for_index(&index, vec![title_field, content_field]);
        query_parser.set_conjunction_by_default();

        Ok(Self {
            index,
            reader,
            query_parser,
            title_field,
            content_field,
            path_field,
        })
    }

    pub fn doc_field_triple(&self) -> (Field, Field, Field) {
        (self.title_field, self.content_field, self.path_field)
    }

    pub fn reload(&mut self) -> Result<()> {
        self.reader.reload()?;
        Ok(())
    }

    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let searcher = self.reader.searcher();
        let query = self.query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (score, doc_address) in top_docs {
            let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;

            let title = retrieved_doc
                .get_first(self.title_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            let path_str = retrieved_doc
                .get_first(self.path_field)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let path_buf = PathBuf::from(&path_str);

            let (content, snippet_highlights) =
                self.get_snippet(&searcher, &*query, query_str, &retrieved_doc, &path_buf)?;

            results.push(SearchResult {
                title,
                content,
                snippet_highlights,
                path: path_buf,
                score,
            });
        }

        Ok(results)
    }

    fn get_snippet(
        &self,
        searcher: &Searcher,
        query: &dyn Query,
        query_str: &str,
        doc: &TantivyDocument,
        path: &Path,
    ) -> Result<(String, Vec<(usize, usize)>)> {
        let indexed_body = trim_chars_clone(
            &join_stored_field(doc, self.content_field),
            SNIPPET_SOURCE_MAX_CHARS,
        );
        let disk_body = load_disk_plain(path).unwrap_or_default();
        let disk_body = trim_chars_clone(&disk_body, SNIPPET_SOURCE_MAX_CHARS);
        let full_body = if !indexed_body.trim().is_empty() {
            indexed_body
        } else {
            disk_body
        };

        let mut snippet_generator = SnippetGenerator::create(searcher, query, self.content_field)?;
        snippet_generator.set_max_num_chars(360);
        let snippet = snippet_generator.snippet_from_doc(doc);
        let fragment = snippet.fragment().to_string();
        let highlights: Vec<(usize, usize)> = snippet
            .highlighted()
            .iter()
            .map(|r| (r.start, r.end))
            .collect();
        if !fragment.trim().is_empty() {
            return Ok(finalize_snippet(fragment, highlights));
        }
        if !full_body.trim().is_empty() {
            let (t, h) = manual_excerpt(&full_body, query_str, 80, 360);
            return Ok(finalize_snippet(t, h));
        }
        let mut gen_title = SnippetGenerator::create(searcher, query, self.title_field)?;
        gen_title.set_max_num_chars(160);
        let sn_title = gen_title.snippet_from_doc(doc);
        if !sn_title.fragment().trim().is_empty() {
            let fragment = sn_title.fragment().to_string();
            let highlights = sn_title
                .highlighted()
                .iter()
                .map(|r| (r.start, r.end))
                .collect();
            return Ok(finalize_snippet(fragment, highlights));
        }
        Ok((String::new(), Vec::new()))
    }

    pub fn get_writer(&self) -> Result<IndexWriter> {
        Ok(self.index.writer(50_000_000)?)
    }
}
