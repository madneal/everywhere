use eframe::egui;
use std::sync::Arc;
use tokio::sync::Mutex;

mod app;
mod search;
mod parser;
mod ui;
mod utils;

use app::DocumentSearchApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    // 创建应用
    let app = DocumentSearchApp::new().await?;

    // 启动GUI
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("文档搜索器"),
        ..Default::default()
    };

    eframe::run_native(
        "Document Search",
        options,
        Box::new(|_cc| Box::new(app)),
    )
        .map_err(|e| e.into())
}