use std::path::PathBuf;

use anyhow::Result;
use egui;
use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::{
    model::{
        fastnote_folderentry::FolderEntry, fastnote_manifest::Manifest, fastnote_page::FastnotePage,
    },
    storage::SharedStorage,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemType {
    Project,
    Folder,
    File,
    Page,
}

#[derive(Clone)]
pub struct FastnoteProject {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub children: Vec<FolderEntry>,
    pub storage: SharedStorage,
}

impl std::fmt::Debug for FastnoteProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastnoteProject")
            .field("path", &self.path)
            .field("manifest", &self.manifest)
            .field("children", &self.children)
            .finish_non_exhaustive()
    }
}

impl FastnoteProject {
    #[deprecated(since = "0.2.0", note = "use sm instead")]
    pub fn save(&self) -> anyhow::Result<()> {
        let manifest_path = self.path.join("manifest.json");
        // let json = serde_json::to_string_pretty(&self.manifest)?;
        // save_persistent_data(manifest_path, &json);
        self.storage.save_manifest(&manifest_path, &self.manifest)?;
        //rep
        Ok(())
    }

    pub fn create_blank(
        storage: SharedStorage,
        path: PathBuf,
        name: String,
        color: Color32,
    ) -> anyhow::Result<Self> {
        std::fs::create_dir_all(&path)?;

        let manifest = Manifest {
            r#type: ItemType::Project,
            name,
            color,
            is_open: true,
            order: vec![],
        };

        let project = Self {
            path,
            manifest,
            children: vec![],
            storage: storage,
        };
        project.save()?;
        Ok(project)
    }

    #[deprecated(since = "0.2.0", note = "use sm instead")]
    pub fn load(storage: SharedStorage, path: PathBuf) -> anyhow::Result<Self> {
        let manifest_path = path.join("manifest.json");
        // let json = load_persistent_data(manifest_path)?;
        // let manifest: Manifest = serde_json::from_str(&json)?;
        let manifest = storage.load_manifest(&manifest_path)?;
        let mut children = vec![];
        // Parcourir les entrées du dossier
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();

            // On ne charge que les dossiers contenant un manifest.json
            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                children.push(FolderEntry::load(storage.clone(), child_path)?);
            }
        }

        let s = Self {
            path,
            manifest,
            children,
            storage,
        };
        Ok(s)
    }
    pub fn get_name(&self) -> String {
        self.manifest.name.clone()
    }
    pub fn get_color(&self) -> Color32 {
        self.manifest.color
    }

    pub fn set_name(&mut self, name: String) -> Result<()> {
        self.manifest.name = name;
        self.save()
    }

    pub fn set_color(&mut self, color: Color32) -> Result<()> {
        self.manifest.color = color;
        self.save()
    }

    pub fn update_page_order(&mut self) {
        // --- Synchronisation manifest <-> filesystem ---

        // 1) Pages réellement présentes dans le filesystem
        let fs_pages: Vec<String> = self
            .children
            .iter()
            .map(|p| {
                p.get_path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .collect();

        // 2) Ordre du manifest (peut être vide si ancien projet)
        let mut order = self.manifest.order.clone();

        // 3) Si ancien projet → ordre naturel
        if order.is_empty() {
            order = fs_pages.clone();
        }

        // 4) Reconstruire la liste ordonnée
        let mut final_children = Vec::new();

        // Pages dans l'ordre du manifest
        for name in &order {
            if let Some(page) = self
                .children
                .iter()
                .find(|p| p.get_path().file_name().unwrap().to_str().unwrap() == name)
            {
                final_children.push(page.clone());
            }
        }

        // 5) Ajouter les pages qui existent mais ne sont pas dans l'ordre
        for page in &self.children {
            let name = page
                .get_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            if !order.contains(&name) {
                final_children.push(page.clone());
            }
        }

        // 6) Mettre à jour le manifest (supprime les pages inexistantes)
        self.manifest.order = final_children
            .iter()
            .map(|p| {
                p.get_path()
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .collect();
        self.children = final_children;
        let res = self.save();
        if let Err(err) = res {
            println!("Error while saving order page: {:?}", err);
        }
    }
}
