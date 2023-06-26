use crate::skin::ButtonType;
use crossbeam_channel::Receiver;
use crossbeam_queue::ArrayQueue;
use serde::Deserialize;
use std::{
    collections::HashSet,
    io::{Read, Write},
    net::TcpStream,
    sync::Arc,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[derive(Default)]
pub struct ControllerState {
    pub buttons: HashSet<ButtonType>,
    pub ls: StickState,
    pub rs: StickState,
}

#[derive(Deserialize, Debug)]
struct Message {
    id: u8,
    bs: u32,
    ls: StickState,
    rs: StickState,
}

#[derive(Deserialize, Copy, Clone, Default, Debug)]
pub struct StickState {
    pub x: f32,
    pub y: f32,
}

const SIXTIETH: Duration = Duration::from_millis(16);

pub fn run_net(
    queue: Arc<ArrayQueue<ControllerState>>,
    addr: String,
    stop: Receiver<()>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let addr = format!("{addr}:2579"); // configurable later :)
        let mut now;
        let mut stream;
        let mut buf = [0; 128];
        'outer: loop {
            stream = TcpStream::connect(&addr).unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs_f32(0.5)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::from_secs_f32(0.5)))
                .unwrap();
            loop {
                now = Instant::now();
                let e = stream.write(b"1"); // probably have 'commands' for sys side later, for now just Anything
                if let Err(_) = e {
                    // recreate connection if timeout or conn lost
                    continue 'outer;
                }
                let len = stream.read(&mut buf);
                if let Err(_) = len {
                    continue 'outer;
                }
                let message = serde_json::from_slice::<Vec<Message>>(&buf[..len.unwrap()]);
                if let Ok(msg) = message {
                    for state in msg {
                        let map = state_to_map(state.bs);
                        #[cfg(debug_assertions)]
                        println!("{map:?} {:?} {:?}", state.ls, state.rs);
                        let cs = ControllerState {
                            buttons: map,
                            ls: state.ls,
                            rs: state.rs,
                        };
                        queue.force_push(cs);
                    }
                } else if let Err(e) = message {
                    println!("{e:?}");
                }
                buf.fill(0);
                if stop.try_recv().is_ok() {
                    break 'outer;
                }
                if Instant::now() - now < SIXTIETH {
                    thread::sleep(Instant::now() - now);
                }
            }
        }
    })
}

fn state_to_map(state: u32) -> HashSet<ButtonType> {
    use ButtonType::*;
    const BUTTONS_ORDER: [ButtonType; 20] = [
        A, B, X, Y, Ls, Rs, L, R, Zl, Zr, Plus, Minus, Left, Up, Right, Down, Lsl, Lsr, Rsl, Rsr,
    ];
    let mut r = HashSet::new();
    for i in (0..BUTTONS_ORDER.len()).map(|i| if i > 15 { i + 8 } else { i }) {
        if (state & (1 << i)) != 0 {
            r.insert(BUTTONS_ORDER[i]);
        }
    }
    r
}
