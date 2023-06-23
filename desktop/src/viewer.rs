use crate::{
    config::Config,
    skin::{bg_dims, Skin},
};
use anyhow::Result;
use macroquad::{prelude::*, Window};

fn gen_conf(dims: (i32, i32)) -> Conf {
    Conf {
        window_title: String::from("periscope"),
        window_resizable: false,
        window_width: dims.0,
        window_height: dims.1,
        ..Default::default()
    }
}

pub fn run_viewer(cfg: Config) -> Result<()> {
    let dims = bg_dims(cfg.skin.as_ref().unwrap())?;
    Window::from_config(gen_conf(dims), async {
        if let Err(e) = viewer_impl(cfg).await {
            eprintln!("{e:?}");
            std::process::exit(1);
        }
    });
    Ok(())
}

async fn viewer_impl(cfg: Config) -> Result<()> {
    let s = Skin::open(&cfg.skin.unwrap())?;
    loop {
        clear_background(BLACK);
        draw_texture(s.background, 0.0, 0.0, WHITE);
        next_frame().await;
    }
}
