use crate::search::{DocumentIndexer, SearchEngine};
use crate::ui::MainWindow;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct DocumentSearchApp {
    main_window: MainWindow,
}

impl DocumentSearchApp {
    pub fn new() -> Result<Self> {
        let index_dir = resolve_index_dir();
        let search_engine = Arc::new(parking_lot::Mutex::new(SearchEngine::new(
            index_dir.to_string_lossy().as_ref(),
        )?));
        let indexer = Arc::new(DocumentIndexer::new(
            Arc::clone(&search_engine),
            index_dir.clone(),
        ));
        let main_window = MainWindow::new(search_engine, indexer, index_dir);
        Ok(Self { main_window })
    }
}

impl eframe::App for DocumentSearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.memory_mut(|mem| mem.stop_text_input());
        }
        self.main_window.update(ctx);
    }
}

/// Resolve a CWD-independent index location: `EVERYWHERE_INDEX_DIR` if set,
/// otherwise the per-user data directory. A legacy `./index` created by older
/// versions is migrated over on first launch.
fn resolve_index_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("EVERYWHERE_INDEX_DIR") {
        let dir = dir.trim();
        if !dir.is_empty() {
            return PathBuf::from(dir);
        }
    }
    let new_dir = default_index_dir();
    migrate_legacy_index(&new_dir);
    new_dir
}

/// CWD-independent default location for the index.
fn default_index_dir() -> PathBuf {
    platform_data_dir()
        .unwrap_or_else(std::env::temp_dir)
        .join("everywhere")
        .join("index")
}

fn platform_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "macos")]
    {
        std::env::var_os("HOME").map(|h| PathBuf::from(h).join("Library/Application Support"))
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var_os("APPDATA").map(PathBuf::from)
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .or_else(|| {
                std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share"))
            })
    }
    #[cfg(not(any(unix, target_os = "windows")))]
    {
        None
    }
}

fn migrate_legacy_index(new_dir: &Path) {
    let legacy = PathBuf::from("./index");
    if new_dir.exists() || !legacy.join("meta.json").exists() {
        return;
    }
    if let Some(parent) = new_dir.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            tracing::warn!("create index parent {:?} failed: {}", parent, e);
            return;
        }
    }
    let migrated = match std::fs::rename(&legacy, new_dir) {
        Ok(()) => true,
        Err(_) => {
            // Cross-device rename fails; fall back to copy + remove.
            copy_dir_recursive(&legacy, new_dir)
                .map(|_| std::fs::remove_dir_all(&legacy))
                .is_ok()
        }
    };
    if migrated {
        tracing::info!("migrated legacy index {:?} -> {:?}", legacy, new_dir);
    } else {
        tracing::warn!(
            "failed to migrate legacy index {:?}; starting fresh at {:?}",
            legacy,
            new_dir
        );
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_index_dir_is_absolute() {
        let dir = default_index_dir();
        assert!(dir.is_absolute(), "index dir must not depend on CWD: {:?}", dir);
    }
}
