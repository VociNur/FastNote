use anyhow::Result;
use egui::Color32;
use std::path::PathBuf;

use crate::{
    model::{
        fastnote_folderentry::FolderEntry, fastnote_manifest::Manifest, fastnote_project::ItemType,
    },
    storage::SharedStorage,
};

#[derive(Clone)]
pub struct FastnoteFolder {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub children: Vec<FolderEntry>,
    pub storage: SharedStorage,
}

impl std::fmt::Debug for FastnoteFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastnoteFolder")
            .field("path", &self.path)
            .field("manifest", &self.manifest)
            .field("children", &self.children)
            .finish_non_exhaustive()
    }
}

impl FastnoteFolder {
    pub fn create_blank(
        path: PathBuf,
        name: String,
        color: Color32,
        storage: SharedStorage,
    ) -> Result<Self> {
        storage.create_dirs(&path)?;

        let manifest = Manifest {
            r#type: ItemType::Folder,
            name,
            color,
            is_open: true,
            order: vec![],
        };

        let folder = Self {
            path,
            manifest,
            children: vec![],
            storage: storage.clone(),
        };
        // folder.save()?;
        storage.save_folder_file(&folder);
        Ok(folder)
    }
    //Code "doublon", TODO: retirer

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
        let res = self.storage.save_folder_file(&self);
        if let Err(err) = res {
            println!("Error while saving order page: {:?}", err);
        }
    }
}
