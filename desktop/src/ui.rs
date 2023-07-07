use crate::config::{config_dir, Config};
use egui_macroquad::egui;
use std::{fs::read_dir, path::Path};

pub fn run_ui(cfg: &mut Config, data: &mut Data) -> bool {
    let mut should_continue = true;
    egui_macroquad::ui(|ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Skin").show_index(
                ui,
                &mut data.selected_skin,
                data.skins.len(),
                |i| data.skins[i].clone(),
            );
            if data.last_selected != data.selected_skin {
                cfg.skin = Some(data.skins[data.selected_skin].clone());
                data.last_selected = data.selected_skin;
            }
            ui.horizontal(|ui| {
                if ui.text_edit_singleline(&mut data.switch_addr).changed() {
                    cfg.switch_addr = Some(data.switch_addr.clone());
                }
                ui.label("Switch IP address");
            });
            if ui
                .checkbox(
                    &mut data.no_show_config,
                    "Don't show this config dialog on next startup",
                )
                .changed()
            {
                cfg.viewer_only = Some(data.no_show_config);
            }
            if ui.button("Launch viewer").clicked() {
                if !cfg.switch_addr.as_ref().is_some_and(|a| !a.is_empty()) {
                    data.have_error = ConfigProblem::Address;
                } else if !cfg.skin.as_ref().is_some_and(|a| !a.is_empty()) {
                    data.have_error = ConfigProblem::Skin;
                } else if cfg.skin.as_ref().is_some()
                    && !Path::new(
                        &Path::new("/home/peri/.config/periscope/")
                            .join(cfg.skin.as_ref().unwrap()),
                    )
                    .exists()
                {
                    // not sure it's possible to get here, but just in case...
                    data.have_error = ConfigProblem::Skin2;
                } else {
                    should_continue = false;
                }
            }
            match data.have_error {
                ConfigProblem::None => {}
                ConfigProblem::Address => {
                    ui.label("Address cannot be empty!");
                }
                ConfigProblem::Skin => {
                    ui.label("Skin cannot be empty!");
                }
                ConfigProblem::Skin2 => {
                    ui.label("Skin directory does not exist!");
                }
            }
        });
    });
    egui_macroquad::draw();
    should_continue
}

pub struct Data {
    skins: Vec<String>,
    selected_skin: usize,
    last_selected: usize,
    switch_addr: String,
    no_show_config: bool,
    have_error: ConfigProblem,
}

enum ConfigProblem {
    None,
    Address,
    Skin,
    Skin2,
}

impl Data {
    pub fn new(cfg: &mut Config) -> Self {
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
        let switch_addr = cfg.switch_addr.clone().unwrap_or_else(String::new);
        let selected_skin = if !cfg.skin.as_ref().unwrap().is_empty() {
            skins
                .iter()
                .position(|s| s == cfg.skin.as_ref().unwrap())
                .unwrap_or(0)
        } else {
            0
        };
        cfg.skin = Some(skins[selected_skin].clone());
        Self {
            skins,
            selected_skin,
            last_selected: selected_skin,
            switch_addr,
            no_show_config: false,
            have_error: ConfigProblem::None,
        }
    }
}

pub fn show_error(err: &str) -> bool {
    let mut ret = false;
    egui_macroquad::ui(|ctx| {
        egui::Window::new("Error!").show(ctx, |ui| {
            ui.label(err);
            ret = ui.button("Ok").clicked();
        });
    });
    egui_macroquad::draw();
    ret
}
