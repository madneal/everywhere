use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

const FILE: &str = "indexed_source_roots.json";

pub fn load_roots(index_dir: &Path) -> Vec<PathBuf> {
    let p = index_dir.join(FILE);
    let Ok(data) = std::fs::read_to_string(p) else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<String>>(&data)
        .unwrap_or_default()
        .into_iter()
        .map(PathBuf::from)
        .collect()
}

pub async fn append_root(index_dir: &Path, root: &Path) -> Result<()> {
    let p = index_dir.join(FILE);
    let mut list: Vec<String> = if p.exists() {
        let s = fs::read_to_string(&p).await.unwrap_or_default();
        serde_json::from_str(&s).unwrap_or_default()
    } else {
        Vec::new()
    };
    let canon = root.canonicalize().unwrap_or_else(|_| root.to_path_buf());
    let entry = canon.to_string_lossy().to_string();
    if !list.iter().any(|x| x == &entry) {
        list.push(entry);
        list.sort();
        fs::create_dir_all(index_dir).await?;
        fs::write(&p, serde_json::to_string_pretty(&list)?).await?;
    }
    Ok(())
}
