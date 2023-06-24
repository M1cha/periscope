use crate::config::config_dir;
use anyhow::Result;
use image::io::Reader;
use macroquad::texture::Texture2D;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
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
            buttons: buttons_from_cfg(&cfg.buttons, &base)?,
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

fn buttons_from_cfg(
    cfg: &ConfigButtons,
    base: &Path,
) -> Result<HashMap<ButtonType, ButtonDisplay>> {
    use ButtonType::*;
    let mut r = HashMap::new();
    r.insert(A, ButtonDisplay::from_cfg(&cfg.a, &base)?);
    r.insert(B, ButtonDisplay::from_cfg(&cfg.b, &base)?);
    r.insert(X, ButtonDisplay::from_cfg(&cfg.x, &base)?);
    r.insert(Y, ButtonDisplay::from_cfg(&cfg.y, &base)?);
    r.insert(Plus, ButtonDisplay::from_cfg(&cfg.plus, &base)?);
    r.insert(Minus, ButtonDisplay::from_cfg(&cfg.minus, &base)?);
    r.insert(Zl, ButtonDisplay::from_cfg(&cfg.zl, &base)?);
    r.insert(Zr, ButtonDisplay::from_cfg(&cfg.zr, &base)?);
    r.insert(L, ButtonDisplay::from_cfg(&cfg.l, &base)?);
    r.insert(R, ButtonDisplay::from_cfg(&cfg.r, &base)?);
    r.insert(Up, ButtonDisplay::from_cfg(&cfg.up, &base)?);
    r.insert(Down, ButtonDisplay::from_cfg(&cfg.down, &base)?);
    r.insert(Left, ButtonDisplay::from_cfg(&cfg.left, &base)?);
    r.insert(Right, ButtonDisplay::from_cfg(&cfg.right, &base)?);
    r.insert(Ls, ButtonDisplay::from_cfg(&cfg.ls, &base)?);
    r.insert(Rs, ButtonDisplay::from_cfg(&cfg.rs, &base)?);
    Ok(r)
}

impl ButtonDisplay {
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
    pub x: f32,
    pub y: f32,
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
    range: f32,
}

pub struct Skin {
    pub background: Texture2D,
    pub buttons: HashMap<ButtonType, ButtonDisplay>,
    pub ls: Stick,
    pub rs: Stick,
}

#[derive(PartialEq, Eq, Hash, Deserialize, Clone, Copy, Debug)]
pub enum ButtonType {
    A,
    B,
    X,
    Y,
    Plus,
    Minus,
    Zl,
    Zr,
    L,
    R,
    Up,
    Down,
    Left,
    Right,
    Ls,
    Rs,
    Lsl,
    Lsr,
    Rsl,
    Rsr,
}

pub struct ButtonDisplay {
    pub tex: Texture2D,
    pub pos: Pos,
}

pub struct Stick {
    pub tex: Texture2D,
    pub pos: Pos,
    pub range: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Background {
    background: String,
}

pub fn bg_dims(name: &str) -> Result<(i32, i32)> {
    let mut p = config_dir().join(name).join("skin.toml");
    let bg: Background = from_str(&fs::read_to_string(&p)?)?;
    p.pop();
    let i = Reader::open(p.join(bg.background))?
        .with_guessed_format()?
        .decode()?;
    Ok((i.width() as i32, i.height() as i32))
}
