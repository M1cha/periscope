use crate::cli::CommandLine;
use anyhow::Result;
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs, io::Write, path::PathBuf};
use toml::{from_str, to_string_pretty};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config {
    pub switch_addr: Option<String>,
    pub skin: Option<String>,
    pub viewer_only: Option<bool>,
}

static DIRS: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("", "periwinkle", "periscope").expect("No valid home directory found!")
});

pub fn config_dir() -> PathBuf {
    DIRS.config_dir().to_path_buf()
}

impl Config {
    pub fn open() -> Result<Self> {
        let p = config_dir();
        let config = p.join("config.toml");
        if !config.exists() {
            fs::create_dir_all(&p)?;
            fs::File::create(&config)?;
        }
        Ok(from_str(&fs::read_to_string(config)?)?)
    }
    pub fn add_cli(&mut self, cli: CommandLine) {
        if self.switch_addr.is_none() {
            self.switch_addr = cli.switch_addr.or(Some(String::new()));
        }
        if self.skin.is_none() {
            self.skin = Some(cli.skin);
        }
        self.viewer_only = self.viewer_only.or(Some(cli.viewer_only));
    }
    pub fn show_config(&self) -> bool {
        !self.viewer_only.is_some_and(|v| v)
    }
    pub fn write(&self) -> Result<()> {
        let p = config_dir();
        let config = p.join("config.toml");
        if !p.exists() {
            fs::create_dir_all(&p)?;
        }
        fs::File::options()
            .write(true)
            .truncate(true)
            .open(config)?
            .write_all(&to_string_pretty(&self)?.as_bytes())?;
        Ok(())
    }
}
