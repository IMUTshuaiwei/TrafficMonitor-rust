#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod monitor;
mod tray;
mod ui;
mod utils;

use app::TrafficMonitorApp;
use config::AppConfig;
use eframe::egui;

fn main() -> eframe::Result<()> {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn"),
    ).init();

    let cfg = AppConfig::load();
    let pos = cfg.general.window_position.clone();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([300.0, 110.0])
            .with_min_inner_size([80.0, 40.0])
            .with_position([pos.x, pos.y])
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(true)
            .with_resizable(true)
            // ← 不在任务栏显示按钮
            .with_taskbar(false)
            .with_title("流量监控"),
        ..Default::default()
    };

    eframe::run_native(
        "流量监控",
        native_options,
        Box::new(|cc| Box::new(TrafficMonitorApp::new(cc))),
    )
}
