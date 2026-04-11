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
use tantivy::{DocAddress, Index, IndexReader, IndexWriter, Searcher, TantivyDocument};

const MIXED_TOKENIZER: &str = "mixed_cjk";
const ROOTS_JSON: &str = "indexed_source_roots.json";
const TOKEN_PATTERN: &str = r"[A-Za-z0-9_]+|[\u{4E00}-\u{9FFF}\u{3400}-\u{4DBF}\u{3040}-\u{309F}\u{30A0}-\u{30FF}]";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub content: String,
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

            let content = self.get_snippet(&searcher, &*query, doc_address)?;

            results.push(SearchResult {
                title,
                content,
                path: PathBuf::from(path_str),
                score,
            });
        }

        Ok(results)
    }

    fn get_snippet(
        &self,
        searcher: &Searcher,
        query: &dyn Query,
        doc_address: DocAddress,
    ) -> Result<String> {
        let snippet_generator = SnippetGenerator::create(searcher, query, self.content_field)?;
        let doc: TantivyDocument = searcher.doc(doc_address)?;
        let snippet = snippet_generator.snippet_from_doc(&doc);
        Ok(snippet.fragment().to_string())
    }

    pub fn get_writer(&self) -> Result<IndexWriter> {
        Ok(self.index.writer(50_000_000)?)
    }
}
