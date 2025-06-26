use crate::search::{SearchEngine, DocumentIndexer};
use crate::ui::MainWindow;
use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

pub struct DocumentSearchApp {
    main_window: MainWindow,
    search_engine: Arc<SearchEngine>,
    indexer: Arc<DocumentIndexer>,
}

impl DocumentSearchApp {
    pub async fn new() -> Result<Self> {
        let search_engine = Arc::new(SearchEngine::new("./index")?);
        let indexer = Arc::new(DocumentIndexer::new(search_engine.clone()));
        let main_window = MainWindow::new(search_engine.clone());

        Ok(Self {
            main_window,
            search_engine,
            indexer,
        })
    }
}

impl eframe::App for DocumentSearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.main_window.update(ctx);
    }
}