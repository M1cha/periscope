use crate::config::config_dir;
use anyhow::Result;
use image::{io::Reader, RgbaImage};
use macroquad::texture::Texture2D;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
};
use toml::from_str;

fn load_image<P: AsRef<Path>>(path: P) -> Result<Texture2D> {
    let img = Reader::open(path)?
        .with_guessed_format()?
        .decode()?
        .into_rgba8();
    Ok(Texture2D::from_rgba8(
        img.width() as u16,
        img.height() as u16,
        img.as_raw(),
    ))
}

impl Skin {
    pub fn open(name: &str) -> Result<Self> {
        let base = config_dir().join(name);
        let cfg = ConfigSkin::open(base.join("skin.toml"))?;
        Ok(Self {
            background: load_image(base.join(&cfg.background))?,
            buttons: Buttons::from_cfg(&cfg.buttons, &base)?,
            ls: Stick::from_cfg(&cfg.ls, &base)?,
            rs: Stick::from_cfg(&cfg.rs, &base)?,
        })
    }
}

impl Stick {
    fn from_cfg(cfg: &ConfigStick, base: &Path) -> Result<Self> {
        Ok(Self {
            pos: cfg.pos,
            range: cfg.range,
            tex: load_image(base.join(&cfg.image))?,
        })
    }
}

impl Buttons {
    fn from_cfg(cfg: &ConfigButtons, base: &Path) -> Result<Self> {
        Ok(Self {
            a: Button::from_cfg(&cfg.a, &base)?,
            b: Button::from_cfg(&cfg.b, &base)?,
            x: Button::from_cfg(&cfg.x, &base)?,
            y: Button::from_cfg(&cfg.y, &base)?,
            plus: Button::from_cfg(&cfg.plus, &base)?,
            minus: Button::from_cfg(&cfg.minus, &base)?,
            home: Button::from_cfg(&cfg.home, &base)?,
            cap: Button::from_cfg(&cfg.cap, &base)?,
            zl: Button::from_cfg(&cfg.zl, &base)?,
            zr: Button::from_cfg(&cfg.zr, &base)?,
            l: Button::from_cfg(&cfg.l, &base)?,
            r: Button::from_cfg(&cfg.r, &base)?,
            up: Button::from_cfg(&cfg.up, &base)?,
            down: Button::from_cfg(&cfg.down, &base)?,
            left: Button::from_cfg(&cfg.left, &base)?,
            right: Button::from_cfg(&cfg.right, &base)?,
            ls: Button::from_cfg(&cfg.ls, &base)?,
            rs: Button::from_cfg(&cfg.rs, &base)?,
        })
    }
}

impl Button {
    fn from_cfg(cfg: &ConfigButton, base: &Path) -> Result<Self> {
        Ok(Self {
            pos: cfg.pos,
            tex: load_image(base.join(&cfg.image))?,
        })
    }
}

impl ConfigSkin {
    fn open(p: PathBuf) -> Result<Self> {
        Ok(from_str(&fs::read_to_string(p)?)?)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigSkin {
    background: String,
    buttons: ConfigButtons,
    ls: ConfigStick,
    rs: ConfigStick,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigButtons {
    a: ConfigButton,
    b: ConfigButton,
    x: ConfigButton,
    y: ConfigButton,
    plus: ConfigButton,
    minus: ConfigButton,
    home: ConfigButton,
    cap: ConfigButton,
    zl: ConfigButton,
    zr: ConfigButton,
    l: ConfigButton,
    r: ConfigButton,
    up: ConfigButton,
    down: ConfigButton,
    left: ConfigButton,
    right: ConfigButton,
    ls: ConfigButton,
    rs: ConfigButton,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigButton {
    image: String,
    pos: Pos,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigStick {
    image: String,
    pos: Pos,
    range: u32,
}

pub struct Skin {
    background: Texture2D,
    buttons: Buttons,
    ls: Stick,
    rs: Stick,
}

pub struct Buttons {
    a: Button,
    b: Button,
    x: Button,
    y: Button,
    plus: Button,
    minus: Button,
    home: Button,
    cap: Button,
    zl: Button,
    zr: Button,
    l: Button,
    r: Button,
    up: Button,
    down: Button,
    left: Button,
    right: Button,
    ls: Button,
    rs: Button,
}

pub struct Button {
    tex: Texture2D,
    pos: Pos,
}

pub struct Stick {
    tex: Texture2D,
    pos: Pos,
    range: u32,
}
