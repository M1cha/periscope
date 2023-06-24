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
        let mut stream = TcpStream::connect(addr).unwrap();
        let mut buf = [0; 128];
        loop {
            now = Instant::now();
            stream.write(b"1").unwrap(); // probably have 'commands' for sys side later, for now just Anything
            let len = stream.read(&mut buf).unwrap();
            let message = serde_json::from_slice::<Message>(&buf[..len]);
            if let Ok(msg) = message {
                let map = state_to_map(msg.bs);
                #[cfg(debug_assertions)]
                println!("{map:?} {:?} {:?}", msg.ls, msg.rs);
                let cs = ControllerState {
                    buttons: map,
                    ls: msg.ls,
                    rs: msg.rs,
                };
                queue.force_push(cs);
            } else if let Err(e) = message {
                println!("{e:?}");
            }
            buf.fill(0);
            if stop.try_recv().is_ok() {
                break;
            }
            if Instant::now() - now < SIXTIETH {
                thread::sleep(Instant::now() - now);
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
