use std::path::{Path, PathBuf};

use anyhow::Result;
use egui;
use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::{
    model::{fastnote_folderentry::FolderEntry, fastnote_manifest::Manifest},
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
    // pub fn save(&self) -> anyhow::Result<()> {
    //     // let manifest_path = self.path.join("manifest.json");
    //     // let json = serde_json::to_string_pretty(&self.manifest)?;
    //     // save_persistent_data(manifest_path, &json);
    //     // self.storage.save_manifest_file(&manifest_path, &self.manifest)?;
    //     //rep
    //     // Ok(())
    //     self.storage.save_project_file(&self)
    // }
    // pub fn load(storage: SharedStorage, path: &Path) -> anyhow::Result<FastnoteProject>{
    //     let project = storage.load_project(path);
    //     project
    // }

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
            storage: storage.clone(),
        };
        storage.save_project_file(&project);
        Ok(project)
    }

    pub fn get_name(&self) -> String {
        self.manifest.name.clone()
    }
    pub fn get_color(&self) -> Color32 {
        self.manifest.color
    }

    pub fn set_name(&mut self, name: String) -> Result<()> {
        self.manifest.name = name;
        self.storage.save_project_file(self)
    }

    pub fn set_color(&mut self, color: Color32) -> Result<()> {
        self.manifest.color = color;
        self.storage.save_project_file(self)
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
        // let res = self.save();
        let res = self.storage.save_project_file(&self);
        if let Err(err) = res {
            println!("Error while saving order page: {:?}", err);
        }
    }
}
