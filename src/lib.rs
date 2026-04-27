#![warn(clippy::all, rust_2018_idioms)]

use egui::Color32;

pub mod app;
pub mod state;
pub mod themes;
pub mod ui;
pub mod icons;
pub mod pen;
pub mod user_project;

pub fn str_hex_to_color(hex: &str)->Color32{
    let hx = hex.trim_start_matches("#");
    let r = u8::from_str_radix(&hx[0..2],16).unwrap();
    let g = u8::from_str_radix(&hx[2..4],16).unwrap();
    let b = u8::from_str_radix(&hx[4..6],16).unwrap();
    Color32::from_rgb(r, g, b)
}
