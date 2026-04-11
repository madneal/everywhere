use anyhow::Result;
use eframe::egui;

mod app;
mod fonts;
mod index_roots;
mod parser;
mod search;
mod ui;
mod utils;

use app::DocumentSearchApp;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let app = DocumentSearchApp::new()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Everywhere"),
        ..Default::default()
    };

    eframe::run_native(
        "Everywhere",
        options,
        Box::new(move |cc| {
            fonts::install_cjk_fonts(&cc.egui_ctx);
            Box::new(app)
        }),
    )
    .map_err(|e| anyhow::anyhow!("{}", e))
}
