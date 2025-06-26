use crate::parser::{parse_pdf, parse_docx, parse_text};
use crate::search::engine::SearchEngine;
use tantivy::{doc, IndexWriter};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use anyhow::Result;
use tokio::fs;

pub struct DocumentIndexer {
    engine: SearchEngine,
}

impl DocumentIndexer {
    pub fn new(engine: SearchEngine) -> Self {
        Self { engine }
    }

    pub async fn index_directory(&self, dir_path: &Path) -> Result<()> {
        let mut writer = self.engine.get_writer()?;

        for entry in WalkDir::new(dir_path).follow_links(true) {
            let entry = entry?;
            if entry.file_type().is_file() {
                if let Some(extension) = entry.path().extension() {
                    match extension.to_str() {
                        Some("pdf") => self.index_pdf(&mut writer, entry.path()).await?,
                        Some("docx") => self.index_docx(&mut writer, entry.path()).await?,
                        Some("txt") | Some("md") => self.index_text(&mut writer, entry.path()).await?,
                        _ => continue,
                    }
                }
            }
        }

        writer.commit()?;
        tracing::info!("索引构建完成");
        Ok(())
    }

    async fn index_pdf(&self, writer: &mut IndexWriter, path: &Path) -> Result<()> {
        tracing::info!("索引PDF文件: {:?}", path);
        let content = parse_pdf(path).await?;
        let title = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        writer.add_document(doc!(
            self.engine.title_field => title,
            self.engine.content_field => content,
            self.engine.path_field => path.to_string_lossy().to_string(),
        ))?;

        Ok(())
    }

    async fn index_docx(&self, writer: &mut IndexWriter, path: &Path) -> Result<()> {
        tracing::info!("索引DOCX文件: {:?}", path);
        let content = parse_docx(path).await?;
        let title = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        writer.add_document(doc!(
            self.engine.title_field => title,
            self.engine.content_field => content,
            self.engine.path_field => path.to_string_lossy().to_string(),
        ))?;

        Ok(())
    }

    async fn index_text(&self, writer: &mut IndexWriter, path: &Path) -> Result<()> {
        tracing::info!("索引文本文件: {:?}", path);
        let content = parse_text(path).await?;
        let title = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();

        writer.add_document(doc!(
            self.engine.title_field => title,
            self.engine.content_field => content,
            self.engine.path_field => path.to_string_lossy().to_string(),
        ))?;

        Ok(())
    }
}