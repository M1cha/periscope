use crate::{
    config::Config,
    net::{run_net, ControllerState},
    skin::{bg_dims, Skin},
};
use anyhow::Result;
use crossbeam_channel::unbounded;
use crossbeam_queue::ArrayQueue;
use macroquad::{prelude::*, Window};
use std::sync::Arc;

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
    let (tx, rx) = unbounded();
    let tx2 = tx.clone();
    let q = Arc::new(ArrayQueue::new(1));
    let addr = cfg.switch_addr.clone().unwrap();
    let h = run_net(Arc::clone(&q), addr, rx);
    Window::from_config(gen_conf(dims), async move {
        if let Err(e) = viewer_impl(cfg, Arc::clone(&q)).await {
            eprintln!("{e:?}");
            tx2.send(()).unwrap();
            std::process::exit(1);
        }
    });
    tx.send(()).unwrap();
    h.join().unwrap();
    Ok(())
}

async fn viewer_impl(cfg: Config, queue: Arc<ArrayQueue<ControllerState>>) -> Result<()> {
    let s = Skin::open(&cfg.skin.unwrap())?;
    let mut cs = ControllerState::default();
    let mut no_frames = 0;
    loop {
        clear_background(BLACK);
        draw_texture(s.background, 0.0, 0.0, WHITE);
        if let Some(frame) = queue.pop() {
            cs = frame;
            no_frames = 0;
        } else {
            no_frames += 1;
        }
        if no_frames > 60 {
            cs = ControllerState::default();
        }
        for button in cs.buttons.iter() {
            let disp = s.buttons.get(&button).unwrap();
            draw_texture(disp.tex, disp.pos.x, disp.pos.y, WHITE);
        }
        next_frame().await;
    }
}
