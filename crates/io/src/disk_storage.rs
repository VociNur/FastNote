//! Adapter disque : la seule implémentation de `Storage` qui touche au système de fichiers.

use std::path::Path;

use fastnote_domain::fastnote_project::Manifest;
use fastnote_domain::storage::Storage;

pub struct DiskStorage;

impl Storage for DiskStorage {
    fn save_manifest(&self, path: &Path, manifest: &Manifest) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(manifest)?;
        crate::persist::save_json_atomic(path, &json)
    }

    fn load_manifest(&self, path: &Path) -> anyhow::Result<Manifest> {
        let json = crate::persist::load_json(path)?;
        Ok(serde_json::from_str(&json)?)
    }

    fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }
}
