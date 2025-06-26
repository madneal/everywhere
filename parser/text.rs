use std::path::Path;
use anyhow::Result;
use tokio::fs;

pub async fn extract_text(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path).await?;
    Ok(content)
}