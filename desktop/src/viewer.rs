use crate::{
    config::Config,
    net::{run_net, ControllerState, NetThreadMsg},
    skin::{bg_dims, Skin},
    ui::{run_ui, show_error, Data},
};
use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
use crossbeam_queue::ArrayQueue;
use macroquad::{
    miniquad::conf::{LinuxBackend, Platform},
    prelude::*,
    Window,
};
use std::{sync::Arc, time::Duration};

fn gen_conf(dims: (i32, i32)) -> Conf {
    Conf {
        window_title: String::from("periscope"),
        window_resizable: false,
        window_width: dims.0,
        window_height: dims.1,
        platform: Platform {
            linux_backend: LinuxBackend::X11WithWaylandFallback,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn run_viewer(cfg: Config) -> Result<()> {
    let (tx, rx) = unbounded();
    let tx2 = tx.clone();
    let q = Arc::new(ArrayQueue::new(1));
    let delay = Duration::from_millis(cfg.delay.unwrap_or(0));
    let h = run_net(Arc::clone(&q), tx.clone(), rx.clone(), delay);
    Window::from_config(gen_conf((400, 200)), async move {
        if let Err(e) = window_loop(cfg, Arc::clone(&q), tx2.clone(), rx).await {
            eprintln!("{e:?}");
            tx2.send(NetThreadMsg::Exit).unwrap();
            std::process::exit(1);
        }
    });
    println!("Exiting...");
    let _ = tx.send(NetThreadMsg::Exit);
    let _ = h.join();
    Ok(())
}

enum Showing {
    ConfigUI,
    ToViewer,
    Viewer,
    Error,
}

use Showing::*;

async fn window_loop(
    mut cfg: Config,
    queue: Arc<ArrayQueue<Vec<ControllerState>>>,
    tx: Sender<NetThreadMsg>,
    rx: Receiver<NetThreadMsg>,
) -> Result<()> {
    let mut s = Skin::default();
    let mut cs = vec![ControllerState::default(); 8];
    let mut no_frames = 0;
    let mut what = if cfg.show_config() {
        ConfigUI
    } else {
        ToViewer
    };
    let mut data = Data::new(&mut cfg);
    let mut err = String::new();
    loop {
        clear_background(BLACK);
        if let Ok(NetThreadMsg::Error(e)) = rx.try_recv() {
            err = e;
            what = Error;
            println!("{err}");
        }
        match what {
            ConfigUI => {
                if !run_ui(&mut cfg, &mut data) {
                    what = ToViewer;
                }
            }
            ToViewer => {
                what = Viewer;
                let dims = bg_dims(&cfg.skin)?;
                request_new_screen_size(dims.0 as f32, dims.1 as f32);
                tx.send(NetThreadMsg::StartCapture(cfg.switch_addr.clone()))
                    .unwrap();
                s = Skin::open(&cfg.skin)?;
                cfg.write()?;
            }
            Viewer => {
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
            Error => {
                if show_error(&err) {
                    break;
                }
            }
        }
        next_frame().await;
    }
    Ok(())
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
