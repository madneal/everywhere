use tantivy::*;
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
use std::path::PathBuf;
use anyhow::Result;
use serde::{Deserialize, Serialize};

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

impl SearchEngine {
    pub fn new(index_path: &str) -> Result<Self> {
        // 创建schema
        let mut schema_builder = Schema::builder();
        let title_field = schema_builder.add_text_field("title", TEXT | STORED);
        let content_field = schema_builder.add_text_field("content", TEXT);
        let path_field = schema_builder.add_text_field("path", STRING | STORED);
        let schema = schema_builder.build();

        // 创建或打开索引
        let index_path = std::path::Path::new(index_path);
        let index = if index_path.exists() {
            Index::open_in_dir(index_path)?
        } else {
            std::fs::create_dir_all(index_path)?;
            Index::create_in_dir(index_path, schema.clone())?
        };

        let reader = index.reader()?;
        let query_parser = QueryParser::for_index(&index, vec![title_field, content_field]);

        Ok(Self {
            index,
            reader,
            query_parser,
            title_field,
            content_field,
            path_field,
        })
    }

    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<SearchResult>> {
        let searcher = self.reader.searcher();
        let query = self.query_parser.parse_query(query_str)?;
        let top_docs = searcher.search(&query, &TopDocs::with_limit(limit))?;

        let mut results = Vec::new();
        for (_score, doc_address) in top_docs {
            let retrieved_doc = searcher.doc(doc_address)?;

            let title = retrieved_doc
                .get_first(self.title_field)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();

            let path_str = retrieved_doc
                .get_first(self.path_field)
                .and_then(|f| f.as_text())
                .unwrap_or("")
                .to_string();

            // 获取匹配的内容片段
            let content = self.get_snippet(&searcher, &query, doc_address)?;

            results.push(SearchResult {
                title,
                content,
                path: PathBuf::from(path_str),
                score: _score,
            });
        }

        Ok(results)
    }

    fn get_snippet(&self, searcher: &Searcher, query: &dyn Query, doc_address: DocAddress) -> Result<String> {
        let snippet_generator = SnippetGenerator::create(searcher, query, self.content_field)?;
        let doc = searcher.doc(doc_address)?;
        let snippet = snippet_generator.snippet_from_doc(&doc);
        Ok(snippet.to_html())
    }

    pub fn get_writer(&self) -> Result<IndexWriter> {
        Ok(self.index.writer(50_000_000)?)
    }
}