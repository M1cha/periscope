use anyhow::Result;
use clap::Parser;
use std::{cell::RefCell, rc::Rc};

mod cli;
mod config;
mod net;
mod skin;
mod ui;
mod viewer;
use config::Config;
use viewer::run_viewer;

fn main() -> Result<()> {
    let args = cli::CommandLine::parse();
    let mut cfg = Config::open()?;
    cfg.add_cli(args);
    let cfg = Rc::new(RefCell::new(cfg));
    if cfg.borrow().show_viewer() {
        ui::run_ui(Rc::clone(&cfg));
    }
    let cfg = Rc::into_inner(cfg).unwrap().into_inner(); // only other ref has been dropped by now
    cfg.write()?;
    run_viewer(cfg)?;
    Ok(())
}
