use crate::config::Config;
use anyhow::Result;
use image::{io::Reader, RgbaImage};
use macroquad::{prelude::*, Window};
use std::path::Path;

fn gen_conf() -> Conf {
    Conf {
        window_title: String::from("periscope"),
        window_resizable: false,
        ..Default::default()
    }
}

pub fn run_viewer(cfg: Config) -> Result<()> {
    // load assets here so we can bail while we still have -> Result
    Window::from_config(gen_conf(), viewer_impl());
    Ok(())
}

async fn viewer_impl() {
    let tex = load_texture("./a.png").await.unwrap();
    loop {
        clear_background(RED);
        draw_circle(50.0, 50.0, 50.0, YELLOW);
        draw_texture(tex, 0.0, 0.0, WHITE);
        next_frame().await;
    }
}

fn load_image<P: AsRef<Path>>(path: P) -> Result<RgbaImage> {
    Ok(Reader::open(path)?
        .with_guessed_format()?
        .decode()?
        .into_rgba8())
}
