//! Persistance JSON générique : écriture atomique (tmp + rename) et lecture.

use crate::paths::PROJECT_DEFAULT_FOLDER;
use std::{fs::{self, File}, path::Path};

use chrono::{Local, NaiveDate};
use flate2::{Compression, bufread::GzEncoder};

use crate::paths::SAVE_DEFAULT_FOLDER;

pub fn filename_region_from_pos(rx: i32, ry: i32) -> String {
    format!("region_{}_{}.json", rx, ry)
}

pub fn my_save_json_data(path: &Path, json: &str) -> anyhow::Result<()> {
    let tmp_path = path.with_extension("tmp");

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&tmp_path, json)?;
    std::fs::rename(&tmp_path, path)?;

    Ok(())
}

pub fn my_load_json_data(path: impl AsRef<Path>) -> anyhow::Result<String> {
    let path = path.as_ref();
    if path.exists() {
        if path.is_file() {
            Ok(std::fs::read_to_string(path).unwrap_or_default())
        } else {
            Err(anyhow::anyhow!("Try to open a folder: {path:?}"))
        }
    } else {
        Err(anyhow::anyhow!("File not found: {path:?}"))
    }
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

