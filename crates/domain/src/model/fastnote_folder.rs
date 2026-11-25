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
    #[deprecated(since = "0.2.0", note = "use sm instead")]
    pub fn load(path: PathBuf, storage: SharedStorage) -> anyhow::Result<Self> {
        let manifest_path = path.join("manifest.json");
        let manifest = storage.load_manifest(&manifest_path)?;

        let mut children = vec![];

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();

            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                children.push(FolderEntry::load(child_path, storage.clone())?);
            }
        }

        let mut s = Self {
            path,
            manifest,
            children,
            storage,
        };
        s.update_page_order();
        Ok(s)
    }

    #[deprecated(since = "0.2.0", note = "use sm instead")]
    pub fn save(&self) -> Result<()> {
        self.storage
            .save_manifest(&self.path.join("manifest.json"), &self.manifest)
    }

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
            storage,
        };
        folder.save()?;
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
        let res = self.save();
        if let Err(err) = res {
            println!("Error while saving order page: {:?}", err);
        }
    }
}
