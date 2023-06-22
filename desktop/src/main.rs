use clap::Parser;

mod cli;

fn main() {
    let args = cli::CommandLine::parse();
    if !args.viewer_only {
        // start cfgui
    }
}
