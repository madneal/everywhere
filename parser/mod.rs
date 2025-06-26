pub mod pdf;
pub mod docx;
pub mod text;

use std::path::Path;
use anyhow::Result;

pub async fn parse_pdf(path: &Path) -> Result<String> {
    pdf::extract_text(path).await
}

pub async fn parse_docx(path: &Path) -> Result<String> {
    docx::extract_text(path).await
}

pub async fn parse_text(path: &Path) -> Result<String> {
    text::extract_text(path).await
}