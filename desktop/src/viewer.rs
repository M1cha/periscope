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
        if let Err(e) = window_loop(cfg, Arc::clone(&q)).await {
            eprintln!("{e:?}");
            tx2.send(()).unwrap();
            std::process::exit(1);
        }
    });
    tx.send(()).unwrap();
    h.join().unwrap();
    Ok(())
}

async fn window_loop(cfg: Config, queue: Arc<ArrayQueue<Vec<ControllerState>>>) -> Result<()> {
    let s = Skin::open(&cfg.skin.unwrap())?;
    let mut cs = vec![ControllerState::default(); 8];
    let mut no_frames = 0;
    let mut running_viewer = true;
    loop {
        clear_background(BLACK);
        if running_viewer {
            if let Some(frame) = queue.pop() {
                cs = frame;
                no_frames = 0;
            } else {
                no_frames += 1;
            }
            if no_frames == 60 {
                cs = vec![ControllerState::default(); 8];
            }
            viewer_impl(&s, &cs[..]);
        }
        next_frame().await;
    }
}

fn viewer_impl(s: &Skin, cs: &[ControllerState]) {
    draw_texture(s.background, 0.0, 0.0, WHITE);
    for (i, state) in cs.iter().enumerate() {
        for button in state.buttons.iter() {
            let disp = s.players[i].buttons.get(&button).unwrap();
            draw_texture(disp.tex, disp.pos.x, disp.pos.y, WHITE);
        }
        let lxm = state.ls.x / 32767.0 * s.players[i].ls.range;
        let rxm = state.rs.x / 32767.0 * s.players[i].rs.range;
        let lym = -state.ls.y / 32767.0 * s.players[i].ls.range;
        let rym = -state.rs.y / 32767.0 * s.players[i].rs.range;
        draw_texture(
            s.players[i].ls.tex,
            s.players[i].ls.pos.x + lxm,
            s.players[i].ls.pos.y + lym,
            WHITE,
        );
        draw_texture(
            s.players[i].rs.tex,
            s.players[i].rs.pos.x + rxm,
            s.players[i].rs.pos.y + rym,
            WHITE,
        );
    }
}
