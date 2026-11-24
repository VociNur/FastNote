use std::{fs, path::PathBuf};

use anyhow::Result;
use egui::Color32;

use crate::fastnote_project::{ItemType, Manifest};
use crate::storage::SharedStorage;
#[derive(Debug, Clone)]
pub struct FastnotePage {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub sm: SharedStorage,
}
impl FolderEntry {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        // Charger le manifest du dossier/fichier
        let manifest_path = path.join("manifest.json");
        let json = load_persistent_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;

        match manifest.r#type {
            ItemType::Folder => Ok(FolderEntry::Folder(FastnoteFolder::load(path)?)),
            ItemType::File => Ok(FolderEntry::File(FastnoteFile::load(path)?)),
            _ => Err(anyhow::anyhow!(
                "FolderEntry::load: type {:?} invalide pour un enfant",
                manifest.r#type
            )),
        }
    }
}
impl FastnotePage {
    pub fn load(sm: SharedStorage, path: PathBuf) -> anyhow::Result<Self> {
        // page.fn → page.json
        let manifest_path = path.join("manifest.json");
        // let json = load_persistent_data(manifest_path)?;
        // let manifest: Manifest = serde_json::from_str(&json)?;
        let manifest = sm.load_manifest(&manifest_path)?;

        Ok(Self { path, manifest, sm })
    }
    pub fn save(&self) -> Result<()> {
        let manifest_path = self.path.join("manifest.json");
        // let json = serde_json::to_string_pretty(&self.manifest)?;
        // save_persistent_data(manifest_path, &json);
        self.sm.save_manifest(&manifest_path, &self.manifest)?;
        Ok(())
    }
    pub fn create_blank(
        sm: SharedStorage,
        page_path: PathBuf,
        name: String,
        color: Color32,
    ) -> Result<Self> {
        let manifest = Manifest {
            r#type: ItemType::Page,
            name,
            color,
            is_open: false,
            order: vec![],
        };

        fs::create_dir_all(page_path.clone())?;
        // 2. Créer les sous-dossiers obligatoires
        fs::create_dir_all(page_path.join("regions"))?;

        fs::create_dir_all(page_path.join("assets"))?;

        let page = Self {
            path: page_path,
            manifest,
            sm,
        };
        page.save()?;
        Ok(page)
    }
}
