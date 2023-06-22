use std::{cell::RefCell, rc::Rc};

use crate::config::Config;
use eframe::egui;

pub fn run_ui(cfg: Rc<RefCell<Config>>) {
    let opts = eframe::NativeOptions {
        centered: true,
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "periscope (configuration)",
        opts,
        Box::new(|_| Box::new(ConfigApp::new(cfg))),
    )
    .unwrap();
}

struct ConfigApp {
    cfg: Rc<RefCell<Config>>,
}

impl ConfigApp {
    fn new(cfg: Rc<RefCell<Config>>) -> Self {
        Self { cfg }
    }
}

impl eframe::App for ConfigApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {});
    }
}
