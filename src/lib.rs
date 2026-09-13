// #![warn(clippy::all, rust_2018_idioms)]

use std::{
    fs::{self, File},
    path::PathBuf,
};

use chrono::{Local, NaiveDate};
use eframe::egui::{self, Color32};
use flate2::{write::GzEncoder, Compression};
use tar::Builder;

use crate::paths::{PROJECT_DEFAULT_FOLDER, SAVE_DEFAULT_FOLDER};

// use egui::Color32;
pub mod app;
pub mod edition;
pub mod errors;
pub mod event_managers;
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

fn get_last_save() -> Option<NaiveDate> {
    let save_folder = get_working_path().join(SAVE_DEFAULT_FOLDER);
    if fs::create_dir_all(&save_folder).is_err() {
        return None;
    }
    let entries = fs::read_dir(save_folder).ok()?;
    let mut latest: Option<NaiveDate> = None;

    for entry in entries.flatten() {
        let path = entry.path();
        let filename = path.file_name()?.to_string_lossy();

        // On attend un format du type "2026-09-12.zip"
        if let Some(date_str) = filename.strip_suffix(".zip") {
            if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
                if latest.map_or(true, |d| date > d) {
                    latest = Some(date);
                }
            }
        }
    }

    latest
}

pub fn create_backup_notes_directory() -> anyhow::Result<()> {
    let save_folder = get_working_path().join(SAVE_DEFAULT_FOLDER);
    fs::create_dir_all(&save_folder)?;

    let today = Local::now().format("%Y-%m-%d").to_string();
    let backup_path = save_folder.join(format!("{}.tar.gz", today));

    let tar_gz = File::create(&backup_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    tar.append_dir_all("projects", get_working_path().join(PROJECT_DEFAULT_FOLDER))?;

    println!("Backup created: {:?}", backup_path);
    Ok(())
}

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

// fn has_persisent_data(path: PathBuf) -> bool {
//     path.exists()
// }

fn load_persistent_data(path: PathBuf) -> anyhow::Result<String> {
    if path.exists() {
        if path.is_file() {
            let json = std::fs::read_to_string(path).unwrap_or_default();
            Ok(json)
        } else {
            println!("Error: try to open a folder");
            Err(anyhow::anyhow!("Try to open a folder"))
        }
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
fn distance_sq(a: egui::Pos2, b: egui::Pos2) -> f32 {
    a.distance_sq(b)
}
pub fn is_valid_folder_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    // Interdits sur Windows + Linux
    let forbidden = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];

    !name.chars().any(|c| forbidden.contains(&c))
}
pub fn folder_exists(parent: &PathBuf, name: &str) -> bool {
    parent.join(name).exists()
}

// pub fn zip_directory(src_dir: &str, dst_file: &str) -> zip::result::ZipResult<()> {
//     let path = std::path::Path::new(src_dir);
//     let file = File::create(dst_file)?;
//     let mut zip = zip::ZipWriter::new(file);

//     let options = FileOptions::default()
//         .compression_method(zip::CompressionMethod::Deflated)
//         .unix_permissions(0o755);

//     for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
//         let entry_path = entry.path();
//         let name = entry_path.strip_prefix(path).unwrap();

//         if entry_path.is_file() {
//             zip.start_file(name.to_string_lossy(), options)?;
//             let mut f = File::open(entry_path)?;
//             std::io::copy(&mut f, &mut zip)?;
//         } else if !name.as_os_str().is_empty() {
//             zip.add_directory(name.to_string_lossy(), options)?;
//         }
//     }

//     zip.finish()?;
//     Ok(())
// }
