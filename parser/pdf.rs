use std::path::Path;
use anyhow::Result;
use tokio::task;

pub async fn extract_text(path: &Path) -> Result<String> {
    let path = path.to_owned();
    task::spawn_blocking(move || {
        let bytes = std::fs::read(&path)?;
        let content = pdf_extract::extract_text_from_mem(&bytes)?;
        Ok(content)
    }).await?
}