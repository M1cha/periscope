mod config_skin;
mod display_skin;

pub use display_skin::{ButtonType, Skin};

use crate::config::config_dir;
use anyhow::Result;
use image::io::Reader;
use macroquad::texture::Texture2D;
use serde::Deserialize;
use std::{fs, path::Path};
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

fn default_image() -> Texture2D {
    Texture2D::from_rgba8(1, 1, &[0, 0, 0, 0])
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug)]
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
