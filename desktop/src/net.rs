use crate::skin::ButtonType;
use crossbeam_channel::{Receiver, Sender};
use crossbeam_queue::ArrayQueue;
use serde::Deserialize;
use std::{
    collections::HashSet,
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    sync::Arc,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[derive(Default, Clone)]
pub struct ControllerState {
    pub id: u8,
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

pub enum NetThreadMsg {
    StartCapture,
    StopCapture,
    Error(&'static str),
    Exit,
}

const SIXTIETH: Duration = Duration::from_millis(16);

pub fn run_net(
    queue: Arc<ArrayQueue<Vec<ControllerState>>>,
    addr: String,
    tx: Sender<NetThreadMsg>,
    rx: Receiver<NetThreadMsg>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let addr = format!("{addr}:2579"); // configurable later :)
        let mut now;
        let mut stream;
        let mut buf = [0; 810];
        let addr = addr.parse();
        if addr.is_err() {
            tx.send(NetThreadMsg::Error("Invalid IP address!")).unwrap();
            return;
        }
        let addr: SocketAddr = addr.unwrap();
        let mut already_capturing = false;
        'outer: loop {
            while !already_capturing {
                if let Ok(m) = rx.try_recv() {
                    match m {
                        NetThreadMsg::Exit => break 'outer,
                        NetThreadMsg::StartCapture => {
                            already_capturing = true;
                            break;
                        }
                        _ => {}
                    }
                }
            }
            if let Ok(s) = TcpStream::connect_timeout(&addr, Duration::from_secs(15)) {
                stream = s;
            } else {
                tx.send(NetThreadMsg::Error("Failed to connect to switch!"))
                    .unwrap();
                return;
            }
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
                let len = len.unwrap();
                let message = serde_json::from_slice::<Vec<Message>>(&buf[..len]);
                if let Ok(msg) = message {
                    let cstates = msg
                        .iter()
                        .map(|state| {
                            let map = state_to_map(state.bs);
                            #[cfg(debug_assertions)]
                            println!("{map:?} {:?} {:?}", state.ls, state.rs);
                            let cs = ControllerState {
                                id: state.id,
                                buttons: map,
                                ls: state.ls,
                                rs: state.rs,
                            };
                            cs
                        })
                        .collect::<Vec<_>>();
                    queue.force_push(cstates);
                } else if let Err(e) = message {
                    println!("{}", String::from_utf8_lossy(&buf[..len]));
                    println!("{e:?}");
                }
                buf.fill(0);
                if let Ok(m) = rx.try_recv() {
                    match m {
                        NetThreadMsg::Exit => break 'outer,
                        NetThreadMsg::StopCapture => {
                            already_capturing = false;
                            continue 'outer;
                        }
                        _ => {}
                    }
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
