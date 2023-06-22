use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
pub struct CommandLine {
    #[arg(long, short = 'a', value_name = "IP_ADDRESS")]
    /// IP address of switch running sys-scope
    pub switch_addr: Option<String>,
    #[arg(long, short = 's', default_value_t = String::from("default"))]
    /// Name of viewer skin
    pub skin: String,
    #[arg(long)]
    /// Skip the configuration dialog and start only the viewer
    pub viewer_only: bool,
}
