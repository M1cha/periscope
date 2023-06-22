use crate::config::{config_dir, Config};
use eframe::egui;
use std::{cell::RefCell, fs::read_dir, rc::Rc};

pub fn run_ui(cfg: Rc<RefCell<Config>>) {
    let opts = eframe::NativeOptions {
        centered: true,
        initial_window_size: Some(egui::vec2(400.0, 200.0)),
        min_window_size: Some(egui::vec2(400.0, 200.0)),
        resizable: false,
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
    skins: Vec<String>,
    selected_skin: usize,
    last_selected: usize,
    switch_addr: String,
    no_show_config: bool,
}

impl ConfigApp {
    fn new(cfg: Rc<RefCell<Config>>) -> Self {
        let mut skins: Vec<_> = read_dir(config_dir())
            .expect("Could not read config directory!")
            .flatten()
            .filter_map(|d| {
                if d.file_type().unwrap().is_dir() {
                    if let Ok(f) = d.file_name().into_string() {
                        Some(f)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        if skins.is_empty() {
            skins.push(String::new());
        }
        let switch_addr = cfg.borrow().switch_addr.clone().unwrap_or_else(String::new);
        let selected_skin = if !cfg.borrow().skin.as_ref().unwrap().is_empty() {
            skins
                .iter()
                .position(|s| s == cfg.borrow().skin.as_ref().unwrap())
                .unwrap_or(0)
        } else {
            0
        };
        Self {
            cfg,
            skins,
            selected_skin,
            last_selected: selected_skin,
            switch_addr,
            no_show_config: false,
        }
    }
}

impl eframe::App for ConfigApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Skin").show_index(
                ui,
                &mut self.selected_skin,
                self.skins.len(),
                |i| &self.skins[i],
            );
            if self.last_selected != self.selected_skin {
                self.cfg.borrow_mut().skin = Some(self.skins[self.selected_skin].clone());
                self.last_selected = self.selected_skin;
            }
            ui.horizontal(|ui| {
                if ui.text_edit_singleline(&mut self.switch_addr).changed() {
                    self.cfg.borrow_mut().switch_addr = Some(self.switch_addr.clone());
                }
                ui.label("Switch IP address");
            });
            if ui
                .checkbox(
                    &mut self.no_show_config,
                    "Don't show this config dialog on next startup",
                )
                .changed()
            {
                self.cfg.borrow_mut().viewer_only = Some(self.no_show_config);
            }
            if ui.button("Launch viewer").clicked() {
                frame.close();
            }
        });
    }
}
