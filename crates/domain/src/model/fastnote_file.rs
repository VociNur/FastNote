use crate::{
    model::{fastnote_manifest::Manifest, fastnote_page::FastnotePage, fastnote_project::ItemType},
    storage::SharedStorage,
};
use anyhow::Result;
use egui::Color32;
use std::path::PathBuf;

#[derive(Clone)]
pub struct FastnoteFile {
    pub children: Vec<FastnotePage>,
    pub manifest: Manifest,
    pub path: PathBuf,
    pub storage: SharedStorage,
}

impl std::fmt::Debug for FastnoteFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FastnoteFile")
            .field("path", &self.path)
            .field("manifest", &self.manifest)
            .field("children", &self.children)
            .finish_non_exhaustive()
    }
}

impl FastnoteFile {
    pub fn create_blank(
        path: PathBuf,
        name: String,
        color: Color32,
        storage: SharedStorage,
    ) -> Result<Self> {
        storage.create_dirs(&path)?;

        let manifest = Manifest {
            r#type: ItemType::File,
            name,
            color,
            is_open: false,
            order: vec![],
        };

        let file = Self {
            path,
            manifest,
            children: vec![],
            storage: storage.clone(),
        };
        storage.save_file_file(&file)?;
        Ok(file)
    }

    pub fn update_page_order(&mut self) {
        // --- Synchronisation manifest <-> filesystem ---

        // 1) Pages réellement présentes dans le filesystem
        let fs_pages: Vec<String> = self
            .children
            .iter()
            .map(|p| p.path.file_name().unwrap().to_str().unwrap().to_owned())
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
                .find(|p| p.path.file_name().unwrap().to_str().unwrap() == name)
            {
                final_children.push(page.clone());
            }
        }

        // 5) Ajouter les pages qui existent mais ne sont pas dans l'ordre
        for page in &self.children {
            let name = page.path.file_name().unwrap().to_str().unwrap().to_owned();
            if !order.contains(&name) {
                final_children.push(page.clone());
            }
        }

        // 6) Mettre à jour le manifest (supprime les pages inexistantes)
        self.manifest.order = final_children
            .iter()
            .map(|p| p.path.file_name().unwrap().to_str().unwrap().to_owned())
            .collect();
        self.children = final_children;
        // let res = self.save();
        let res = self.storage.save_file_file(&self);
        if let Err(err) = res {
            println!("Error while saving order page: {:?}", err);
        }
    }
}
