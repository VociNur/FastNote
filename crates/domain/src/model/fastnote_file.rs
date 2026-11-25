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
    #[deprecated(since = "0.2.0", note = "use sm instead")]
    pub fn load(sm: SharedStorage, path: PathBuf) -> anyhow::Result<Self> {
        let manifest_path = path.join("manifest.json");
        let manifest = sm.load_manifest(&manifest_path)?;

        let mut children = vec![];
        // Parcourir les sous-dossiers du fichier
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();
            // On ne charge que les dossiers contenant un manifest.json
            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                // Lire le manifest du dossier enfant
                let child_manifest = sm.load_manifest(&child_path.join("manifest.json"))?;

                match child_manifest.r#type {
                    ItemType::Page => {
                        // Vérifier la structure obligatoire d'une Page
                        let regions = child_path.join("regions");
                        let assets = child_path.join("assets");

                        if !regions.exists() || !assets.exists() {
                            // Page corrompue → on ne la charge pas
                            // (tu adapteras la méthode exacte si besoin)
                            // app.push_unsafe_minute_error(format!(
                            //     "Page {:?} was corrupted (missing regions/ or assets/)",
                            //     child_path
                            // ));
                            println!(
                                "Page {:?} was corrupted (missing regions/ or assets/)",
                                child_path
                            );
                            continue;
                        }

                        // Charger la page
                        let page = FastnotePage::load(sm.clone(), child_path.clone())?;
                        children.push(page);
                    }

                    other => {
                        // Un File ne doit contenir que des Pages
                        println!("File {:?} contains invalid child type {:?}", path, other);
                    }
                }
            }
        }

        let mut s = Self {
            path,
            manifest,
            children,
            storage: sm,
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
            storage: storage,
        };
        file.save()?;
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
        let res = self.save();
        if let Err(err) = res {
            println!("Error while saving order page: {:?}", err);
        }
    }
}
