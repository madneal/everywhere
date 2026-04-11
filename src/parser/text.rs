use std::path::Path;
use anyhow::Result;
use tokio::fs;

pub async fn extract_text(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path).await?;
    Ok(content)
}

pub async fn extract_text_lossy(path: &Path) -> Result<String> {
    let bytes = fs::read(path).await?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}
