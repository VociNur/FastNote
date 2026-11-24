//! Persistance JSON générique : écriture atomique (tmp + rename) et lecture.

use std::path::Path;

pub fn save_json_atomic(path: &Path, json: &str) -> anyhow::Result<()> {
    let tmp_path = path.with_extension("tmp");

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&tmp_path, json)?;
    std::fs::rename(&tmp_path, path)?;

    Ok(())
}

/// Écrit `json` dans `path`, en loggant l'erreur au lieu de la propager.
pub fn save_json(path: impl AsRef<Path>, json: &str) {
    let path = path.as_ref();
    if let Err(e) = save_json_atomic(path, json) {
        println!("Could not save file: {path:?}: {e}");
    }
}

pub fn load_json(path: impl AsRef<Path>) -> anyhow::Result<String> {
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
