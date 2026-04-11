use std::io::Read;
use std::path::Path;
use anyhow::{Context, Result};
use regex::Regex;
use tokio::task;

pub async fn extract_text(path: &Path) -> Result<String> {
    let path = path.to_path_buf();
    task::spawn_blocking(move || extract_sync(&path)).await?
}

fn extract_sync(path: &Path) -> Result<String> {
    let bytes = std::fs::read(path)?;
    let mut archive = zip::ZipArchive::new(std::io::Cursor::new(bytes))?;
    let mut xml = String::new();
    let mut f = archive
        .by_name("word/document.xml")
        .context("missing word/document.xml")?;
    f.read_to_string(&mut xml)?;
    let re = Regex::new(r"<w:t[^>]*>([^<]*)</w:t>")?;
    let mut out = String::new();
    for cap in re.captures_iter(&xml) {
        out.push_str(&cap[1]);
        out.push(' ');
    }
    Ok(out.trim().to_string())
}
