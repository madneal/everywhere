use crate::search::{DocumentIndexer, SearchEngine};
use crate::ui::MainWindow;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

const INDEX_DIR: &str = "./index";

pub struct DocumentSearchApp {
    main_window: MainWindow,
}

impl DocumentSearchApp {
    pub fn new() -> Result<Self> {
        let index_dir = PathBuf::from(INDEX_DIR);
        let search_engine = Arc::new(parking_lot::Mutex::new(SearchEngine::new(INDEX_DIR)?));
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
        self.main_window.update(ctx);
    }
}
