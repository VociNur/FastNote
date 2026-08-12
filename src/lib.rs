// #![warn(clippy::all, rust_2018_idioms)]

use std::path::PathBuf;

use eframe::egui::{self, Color32};

// use egui::Color32;
pub mod app;
pub mod edition;
pub mod file_tree_state;
pub mod gpu;
pub mod icons;
pub mod paths;
pub mod pen;
pub mod projects;
pub mod state;
pub mod strokes;
pub mod stylet;
pub mod themes;
pub mod ui;
mod event_managers;
fn get_working_path() -> std::path::PathBuf {
    dirs::data_dir().unwrap_or_else(|| std::path::PathBuf::from("."))
}

fn aux_save_persistent_data(path: PathBuf, json: &str) -> anyhow::Result<()> {
    let tmp_path = path.with_extension("tmp");

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&tmp_path, json)?;

    // 2. Renommer atomiquement → remplace l'ancien fichier d'un coup
    std::fs::rename(&tmp_path, path)?;

    Ok(())
}

fn save_persistent_data(path: PathBuf, json: &str) {
    if aux_save_persistent_data(path.clone(), json).is_err() {
        println!("Could not save file: {path:?}");
    }
}

fn has_persisent_data(path: PathBuf) -> bool {
    path.exists()
}

fn load_persistent_data(path: PathBuf) -> anyhow::Result<String> {
    if path.exists() {
        let json = std::fs::read_to_string(path).unwrap_or_default();
        Ok(json)
    } else {
        println!("Opening data that doesn't exist");
        println!("File {:?}", path);
        Err(anyhow::anyhow!("File not found"))
    }
}
fn distance_point_to_segment(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let t = (ap.dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
    let closest = a + ab * t;
    (p - closest).length()
}

fn color_to_rgb(color: &Color32) -> u32 {
    ((color.r() as u32) << 24) + ((color.g() as u32) << 16) + ((color.b() as u32) << 8) + 255
}
fn distance_sq(a: egui::Pos2, b:egui::Pos2) -> f32{
    a.distance_sq(b)
}
