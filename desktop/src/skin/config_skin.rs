use anyhow::Result;
use serde::Deserialize;
use std::{fs, path::PathBuf};
use toml::from_str;

use super::Pos;

#[derive(Deserialize, Debug)]
pub(super) struct ConfigSkin {
    pub background: String,
    #[serde(default)]
    pub player1: ConfigPlayer,
    #[serde(default)]
    pub player2: ConfigPlayer,
    #[serde(default)]
    pub player3: ConfigPlayer,
    #[serde(default)]
    pub player4: ConfigPlayer,
    #[serde(default)]
    pub player5: ConfigPlayer,
    #[serde(default)]
    pub player6: ConfigPlayer,
    #[serde(default)]
    pub player7: ConfigPlayer,
    #[serde(default)]
    pub player8: ConfigPlayer,
}

impl ConfigSkin {
    pub fn open(p: PathBuf) -> Result<Self> {
        Ok(from_str(&fs::read_to_string(p)?)?)
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub(super) struct ConfigPlayer {
    pub buttons: ConfigButtons,
    pub ls: ConfigStick,
    pub rs: ConfigStick,
}

#[derive(Deserialize, Debug, Default)]
#[serde(default)]
pub(super) struct ConfigButtons {
    pub a: ConfigButton,
    pub b: ConfigButton,
    pub x: ConfigButton,
    pub y: ConfigButton,
    pub plus: ConfigButton,
    pub minus: ConfigButton,
    pub zl: ConfigButton,
    pub zr: ConfigButton,
    pub l: ConfigButton,
    pub r: ConfigButton,
    pub up: ConfigButton,
    pub down: ConfigButton,
    pub left: ConfigButton,
    pub right: ConfigButton,
    pub ls: ConfigButton,
    pub rs: ConfigButton,
    pub lsl: ConfigButton,
    pub lsr: ConfigButton,
    pub rsl: ConfigButton,
    pub rsr: ConfigButton,
}

#[derive(Deserialize, Debug, Default)]
pub(super) struct ConfigButton {
    pub image: String,
    pub pos: Pos,
}

#[derive(Deserialize, Debug, Default)]
pub(super) struct ConfigStick {
    pub image: String,
    pub pos: Pos,
    pub range: f32,
}
