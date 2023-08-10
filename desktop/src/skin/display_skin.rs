use crate::config::config_dir;
use anyhow::Result;
use macroquad::texture::Texture2D;
use serde::Deserialize;
use std::{collections::HashMap, path::Path};

use super::{config_skin::*, default_image, load_image, Pos};

impl Skin {
    pub fn open(name: &str) -> Result<Self> {
        let base = config_dir().join(name);
        let cfg = ConfigSkin::open(base.join("skin.toml"))?;
        Ok(Self {
            background: load_image(base.join(&cfg.background))?,
            players: [
                PlayerSkin::from_cfg(cfg.player1, &base)?,
                PlayerSkin::from_cfg(cfg.player2, &base)?,
                PlayerSkin::from_cfg(cfg.player3, &base)?,
                PlayerSkin::from_cfg(cfg.player4, &base)?,
                PlayerSkin::from_cfg(cfg.player5, &base)?,
                PlayerSkin::from_cfg(cfg.player6, &base)?,
                PlayerSkin::from_cfg(cfg.player7, &base)?,
                PlayerSkin::from_cfg(cfg.player8, &base)?,
            ],
        })
    }
}

impl Default for Skin {
    fn default() -> Self {
        Self {
            background: default_image(),
            players: [
                PlayerSkin::default(),
                PlayerSkin::default(),
                PlayerSkin::default(),
                PlayerSkin::default(),
                PlayerSkin::default(),
                PlayerSkin::default(),
                PlayerSkin::default(),
                PlayerSkin::default(),
            ],
        }
    }
}

impl PlayerSkin {
    fn from_cfg(cfg: ConfigPlayer, base: &Path) -> Result<Self> {
        Ok(Self {
            buttons: buttons_from_cfg(&cfg.buttons, &base)?,
            ls: Stick::from_cfg(&cfg.ls, &base)?,
            rs: Stick::from_cfg(&cfg.rs, &base)?,
        })
    }
}

impl Default for PlayerSkin {
    fn default() -> Self {
        Self {
            buttons: HashMap::new(),
            ls: Stick::default(),
            rs: Stick::default(),
        }
    }
}

impl Stick {
    fn from_cfg(cfg: &ConfigStick, base: &Path) -> Result<Self> {
        Ok(Self {
            pos: cfg.pos,
            range: cfg.range,
            tex: if cfg.image.is_empty() {
                default_image()
            } else {
                load_image(base.join(&cfg.image))?
            },
        })
    }
}

impl Default for Stick {
    fn default() -> Self {
        Self {
            pos: Pos { x: 0.0, y: 0.0 },
            range: 0.0,
            tex: default_image(),
        }
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
    r.insert(Lsl, ButtonDisplay::from_cfg(&cfg.lsl, &base)?);
    r.insert(Lsr, ButtonDisplay::from_cfg(&cfg.lsr, &base)?);
    r.insert(Rsl, ButtonDisplay::from_cfg(&cfg.rsl, &base)?);
    r.insert(Rsr, ButtonDisplay::from_cfg(&cfg.rsr, &base)?);
    Ok(r)
}

impl ButtonDisplay {
    fn from_cfg(cfg: &ConfigButton, base: &Path) -> Result<Self> {
        Ok(Self {
            pos: cfg.pos,
            tex: if cfg.image.is_empty() {
                default_image()
            } else {
                load_image(base.join(&cfg.image))?
            },
        })
    }
}

pub struct Skin {
    pub background: Texture2D,
    pub players: [PlayerSkin; 8],
}

pub struct PlayerSkin {
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
