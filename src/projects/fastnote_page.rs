use std::{fs, path::PathBuf};

use eframe::egui::Color32;

use crate::{
    load_persistent_data,
    projects::fastnote_project::{ItemType, Manifest},
    save_persistent_data,
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FastnotePage {
    pub path: PathBuf,
    pub manifest: Manifest,
}
impl FastnotePage {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        // page.fn → page.json
        let manifest_path = path.join("manifest.json");
        let json = load_persistent_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;

        Ok(Self { path, manifest })
    }
    pub fn save(&self) -> Result<()> {
        let manifest_path = self.path.join("manifest.json");
        let json = serde_json::to_string_pretty(&self.manifest)?;
        save_persistent_data(manifest_path, &json);
        Ok(())
    }

    pub fn create_blank(page_path: PathBuf, name: String, color: Color32) -> Result<Self> {
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
        };
        page.save()?;
        Ok(page)
    }

}
