use crate::index_roots;
use crate::parser::{parse_docx, parse_pdf, parse_text, parse_text_lossy};
use crate::search::engine::SearchEngine;
use crate::utils::{file_base_name, is_known_text_extension, is_probably_binary_extension, looks_binary_sample};
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tantivy::{doc, IndexWriter};
use tokio::fs;
use walkdir::WalkDir;

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
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_ascii_lowercase();

            if ext == "pdf" {
                self.add_pdf(&mut writer, title_f, content_f, path_f, path)
                    .await?;
                continue;
            }
            if ext == "docx" {
                self.add_docx(&mut writer, title_f, content_f, path_f, path)
                    .await?;
                continue;
            }
            if ext == "md" || ext == "mdx" || ext == "markdown" {
                self.add_markdown(&mut writer, title_f, content_f, path_f, path)
                    .await?;
                continue;
            }
            if is_known_text_extension(&ext) {
                self.add_text_file(&mut writer, title_f, content_f, path_f, path)
                    .await?;
                continue;
            }
            if ext.is_empty() {
                self.add_if_utf8_text(&mut writer, title_f, content_f, path_f, path)
                    .await?;
                continue;
            }
            if is_probably_binary_extension(&ext) {
                continue;
            }
            self.add_if_utf8_text(&mut writer, title_f, content_f, path_f, path)
                .await?;
        }

        writer.commit()?;
        self.engine.lock().reload()?;
        if let Err(e) = index_roots::append_root(&self.index_dir, dir_path).await {
            tracing::warn!("persist indexed root failed: {}", e);
        }
        tracing::info!("index build finished");
        Ok(())
    }

    async fn add_pdf(
        &self,
        writer: &mut IndexWriter,
        title_f: tantivy::schema::Field,
        content_f: tantivy::schema::Field,
        path_f: tantivy::schema::Field,
        path: &Path,
    ) -> Result<()> {
        let content = match parse_pdf(path).await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("skip PDF {:?}: {}", path, e);
                return Ok(());
            }
        };
        if content.trim().is_empty() {
            return Ok(());
        }
        let title = file_base_name(path);
        writer.add_document(doc!(
            title_f => title,
            content_f => content,
            path_f => path.to_string_lossy().as_ref(),
        ))?;
        Ok(())
    }

    async fn add_docx(
        &self,
        writer: &mut IndexWriter,
        title_f: tantivy::schema::Field,
        content_f: tantivy::schema::Field,
        path_f: tantivy::schema::Field,
        path: &Path,
    ) -> Result<()> {
        let content = match parse_docx(path).await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("skip DOCX {:?}: {}", path, e);
                return Ok(());
            }
        };
        if content.trim().is_empty() {
            return Ok(());
        }
        let title = file_base_name(path);
        writer.add_document(doc!(
            title_f => title,
            content_f => content,
            path_f => path.to_string_lossy().as_ref(),
        ))?;
        Ok(())
    }

    async fn add_markdown(
        &self,
        writer: &mut IndexWriter,
        title_f: tantivy::schema::Field,
        content_f: tantivy::schema::Field,
        path_f: tantivy::schema::Field,
        path: &Path,
    ) -> Result<()> {
        let content = match crate::parser::parse_markdown(path).await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("skip markdown {:?}: {}", path, e);
                return Ok(());
            }
        };
        if content.trim().is_empty() {
            return Ok(());
        }
        let title = file_base_name(path);
        writer.add_document(doc!(
            title_f => title,
            content_f => content,
            path_f => path.to_string_lossy().as_ref(),
        ))?;
        Ok(())
    }

    async fn add_text_file(
        &self,
        writer: &mut IndexWriter,
        title_f: tantivy::schema::Field,
        content_f: tantivy::schema::Field,
        path_f: tantivy::schema::Field,
        path: &Path,
    ) -> Result<()> {
        let content = match parse_text(path).await {
            Ok(c) => c,
            Err(_) => parse_text_lossy(path).await?,
        };
        if content.trim().is_empty() {
            return Ok(());
        }
        let title = file_base_name(path);
        writer.add_document(doc!(
            title_f => title,
            content_f => content,
            path_f => path.to_string_lossy().as_ref(),
        ))?;
        Ok(())
    }

    async fn add_if_utf8_text(
        &self,
        writer: &mut IndexWriter,
        title_f: tantivy::schema::Field,
        content_f: tantivy::schema::Field,
        path_f: tantivy::schema::Field,
        path: &Path,
    ) -> Result<()> {
        let sample_len = 64usize * 1024;
        let bytes = fs::read(path).await?;
        let sample = if bytes.len() > sample_len {
            &bytes[..sample_len]
        } else {
            &bytes[..]
        };
        if looks_binary_sample(sample) {
            return Ok(());
        }
        let content = String::from_utf8_lossy(&bytes).into_owned();
        if content.trim().is_empty() {
            return Ok(());
        }
        let title = file_base_name(path);
        writer.add_document(doc!(
            title_f => title,
            content_f => content,
            path_f => path.to_string_lossy().as_ref(),
        ))?;
        Ok(())
    }
}
