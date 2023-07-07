#![windows_subsystem = "windows"]
use anyhow::Result;
use clap::Parser;

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
    cfg.write()?;
    run_viewer(cfg)?;
    Ok(())
}
