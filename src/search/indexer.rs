use crate::index_roots;
use crate::parser::{parse_docx, parse_pdf, parse_text, parse_text_lossy};
use crate::search::engine::SearchEngine;
use crate::utils::{file_base_name, is_known_text_extension, is_probably_binary_extension, looks_binary_sample};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tantivy::schema::Field;
use tantivy::{doc, IndexWriter, Term};
use tokio::fs;
use walkdir::WalkDir;

/// Commit every N indexed documents so that a failure near the end of a
/// large folder does not discard the whole batch of work.
const COMMIT_BATCH_SIZE: usize = 500;

pub struct DocumentIndexer {
    engine: Arc<parking_lot::Mutex<SearchEngine>>,
    index_dir: PathBuf,
}

impl DocumentIndexer {
    pub fn new(engine: Arc<parking_lot::Mutex<SearchEngine>>, index_dir: PathBuf) -> Self {
        Self { engine, index_dir }
    }

    pub async fn index_directory(&self, dir_path: &Path) -> Result<()> {
        let (mut writer, title_f, content_f, path_f) = {
            let eng = self.engine.lock();
            let w = eng.get_writer()?;
            let (t, c, p) = eng.doc_field_triple();
            (w, t, c, p)
        };

        let mut pending = 0usize;
        for entry in WalkDir::new(dir_path)
            .follow_links(true)
            .into_iter()
            .filter_entry(|e| {
                if e.file_type().is_dir() {
                    if let Some(name) = e.file_name().to_str() {
                        return !crate::utils::should_skip_dir(name);
                    }
                }
                true
            })
        {
            // A single unreadable entry must not abort the whole indexing run.
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    tracing::warn!("skip unreadable entry: {}", e);
                    continue;
                }
            };
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();

            let res = if ext == "pdf" {
                self.add_pdf(&mut writer, title_f, content_f, path_f, path).await
            } else if ext == "docx" {
                self.add_docx(&mut writer, title_f, content_f, path_f, path).await
            } else if ext == "md" || ext == "mdx" || ext == "markdown" {
                self.add_markdown(&mut writer, title_f, content_f, path_f, path).await
            } else if is_known_text_extension(&ext) {
                self.add_text_file(&mut writer, title_f, content_f, path_f, path).await
            } else if ext.is_empty() {
                self.add_if_utf8_text(&mut writer, title_f, content_f, path_f, path).await
            } else if is_probably_binary_extension(&ext) {
                Ok(false)
            } else {
                self.add_if_utf8_text(&mut writer, title_f, content_f, path_f, path).await
            };

            match res {
                Ok(true) => {
                    pending += 1;
                    if pending >= COMMIT_BATCH_SIZE {
                        if let Err(e) = writer.commit() {
                            tracing::error!("intermediate commit failed: {}", e);
                        }
                        pending = 0;
                    }
                }
                Ok(false) => {}
                Err(e) => tracing::warn!("skip file {:?}: {}", path, e),
            }
        }

        writer.commit()?;
        self.engine.lock().reload()?;
        if let Err(e) = index_roots::append_root(&self.index_dir, dir_path).await {
            tracing::warn!("persist indexed root failed: {}", e);
        }
        tracing::info!("index build finished");
        Ok(())
    }

    /// Upsert: remove any previous document stored at the same path before
    /// adding, so that re-indexing a folder never creates duplicates.
    fn push_document(
        writer: &mut IndexWriter,
        title_f: Field,
        content_f: Field,
        path_f: Field,
        path: &Path,
        content: String,
    ) -> Result<bool> {
        if content.trim().is_empty() {
            return Ok(false);
        }
        let path_str = path.to_string_lossy().into_owned();
        writer.delete_term(Term::from_field_text(path_f, &path_str));
        let title = file_base_name(path);
        writer.add_document(doc!(
            title_f => title,
            content_f => content,
            path_f => path_str.as_str(),
        ))?;
        Ok(true)
    }

    async fn add_pdf(
        &self,
        writer: &mut IndexWriter,
        title_f: Field,
        content_f: Field,
        path_f: Field,
        path: &Path,
    ) -> Result<bool> {
        match parse_pdf(path).await {
            Ok(content) => Self::push_document(writer, title_f, content_f, path_f, path, content),
            Err(e) => {
                tracing::warn!("skip PDF {:?}: {}", path, e);
                Ok(false)
            }
        }
    }

    async fn add_docx(
        &self,
        writer: &mut IndexWriter,
        title_f: Field,
        content_f: Field,
        path_f: Field,
        path: &Path,
    ) -> Result<bool> {
        match parse_docx(path).await {
            Ok(content) => Self::push_document(writer, title_f, content_f, path_f, path, content),
            Err(e) => {
                tracing::warn!("skip DOCX {:?}: {}", path, e);
                Ok(false)
            }
        }
    }

    async fn add_markdown(
        &self,
        writer: &mut IndexWriter,
        title_f: Field,
        content_f: Field,
        path_f: Field,
        path: &Path,
    ) -> Result<bool> {
        match crate::parser::parse_markdown(path).await {
            Ok(content) => Self::push_document(writer, title_f, content_f, path_f, path, content),
            Err(e) => {
                tracing::warn!("skip markdown {:?}: {}", path, e);
                Ok(false)
            }
        }
    }

    async fn add_text_file(
        &self,
        writer: &mut IndexWriter,
        title_f: Field,
        content_f: Field,
        path_f: Field,
        path: &Path,
    ) -> Result<bool> {
        let content = match parse_text(path).await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("non-utf8 text {:?} ({}), falling back to lossy", path, e);
                match parse_text_lossy(path).await {
                    Ok(c) => c,
                    Err(e) => {
                        tracing::warn!("skip text {:?}: {}", path, e);
                        return Ok(false);
                    }
                }
            }
        };
        Self::push_document(writer, title_f, content_f, path_f, path, content)
    }

    async fn add_if_utf8_text(
        &self,
        writer: &mut IndexWriter,
        title_f: Field,
        content_f: Field,
        path_f: Field,
        path: &Path,
    ) -> Result<bool> {
        let sample_len = 64usize * 1024;
        let bytes = match fs::read(path).await {
            Ok(b) => b,
            Err(e) => {
                tracing::warn!("skip unreadable file {:?}: {}", path, e);
                return Ok(false);
            }
        };
        let sample = if bytes.len() > sample_len {
            &bytes[..sample_len]
        } else {
            &bytes[..]
        };
        if looks_binary_sample(sample) {
            return Ok(false);
        }
        let content = String::from_utf8_lossy(&bytes).into_owned();
        Self::push_document(writer, title_f, content_f, path_f, path, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::engine::SearchEngine;

    fn tmp_dir(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "everywhere_test_{}_{}",
            tag,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[tokio::test]
    async fn reindex_same_folder_does_not_duplicate() {
        let root = tmp_dir("root");
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(root.join("note.txt"), "hello unique zebra content").unwrap();

        let index_dir = tmp_dir("index");
        let engine = Arc::new(parking_lot::Mutex::new(
            SearchEngine::new(index_dir.to_str().unwrap()).unwrap(),
        ));
        let indexer = DocumentIndexer::new(Arc::clone(&engine), index_dir.clone());

        indexer.index_directory(&root).await.unwrap();
        indexer.index_directory(&root).await.unwrap();

        let results = engine.lock().search("zebra", 10).unwrap();
        assert_eq!(
            results.len(),
            1,
            "re-indexing the same folder must not duplicate documents"
        );

        let _ = std::fs::remove_dir_all(&root);
        let _ = std::fs::remove_dir_all(&index_dir);
    }
}
