use std::path::PathBuf;

use anyhow::Result;
use eframe::egui;
use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::{load_persistent_data, projects::fastnote_page::FastnotePage, save_persistent_data};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ItemType {
    Project,
    Folder,
    File,
    Page,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub r#type: ItemType,

    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub color: egui::Color32,
    #[serde(default)]
    pub is_open: bool, //osef pour un file/autre
    #[serde(default)]
    pub order: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum FolderEntry {
    Folder(FastnoteFolder),
    File(FastnoteFile),
}

#[derive(Debug, Clone)]
pub struct FastnoteProject {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub children: Vec<FolderEntry>,
}

#[derive(Debug, Clone)]
pub struct FastnoteFolder {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub children: Vec<FolderEntry>,
}

#[derive(Debug, Clone)]
pub struct FastnoteFile {
    pub path: PathBuf,
    pub manifest: Manifest,
    pub children: Vec<FastnotePage>,
}

impl FastnoteProject {
    pub fn save(&self) -> anyhow::Result<()> {
        let manifest_path = self.path.join("manifest.json");
        let json = serde_json::to_string_pretty(&self.manifest)?;
        save_persistent_data(manifest_path, &json);
        Ok(())
    }

    pub fn create_blank(path: PathBuf, name: String, color: Color32) -> anyhow::Result<Self> {
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
        };
        project.save()?;
        Ok(project)
    }

    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let manifest_path = path.join("manifest.json");
        let json = load_persistent_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;
        let mut children = vec![];
        // Parcourir les entrées du dossier
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();

            // On ne charge que les dossiers contenant un manifest.json
            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                children.push(FolderEntry::load(child_path)?);
            }
        }

        let s = Self {
            path,
            manifest,
            children,
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
impl FastnoteFolder {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let manifest_path = path.join("manifest.json");
        let json = load_persistent_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;

        let mut children = vec![];

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();

            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                children.push(FolderEntry::load(child_path)?);
            }
        }

        let mut s = Self {
            path,
            manifest,
            children,
        };
        s.update_page_order();
        Ok(s)
    }

    pub fn save(&self) -> Result<()> {
        let manifest_path = self.path.join("manifest.json");
        let json = serde_json::to_string_pretty(&self.manifest)?;
        save_persistent_data(manifest_path, &json);
        Ok(())
    }

    pub fn create_blank(path: PathBuf, name: String, color: Color32) -> Result<Self> {
        std::fs::create_dir_all(&path)?;

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

impl FastnoteFile {
    pub fn load(path: PathBuf) -> anyhow::Result<Self> {
        let manifest_path = path.join("manifest.json");
        let json = load_persistent_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;

        let mut children = vec![];
        // Parcourir les sous-dossiers du fichier
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();
            // On ne charge que les dossiers contenant un manifest.json
            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                // Lire le manifest du dossier enfant
                let json = load_persistent_data(child_path.join("manifest.json"))?;
                let child_manifest: Manifest = serde_json::from_str(&json)?;

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
                        let page = FastnotePage::load(child_path.clone())?;
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
        };
        s.update_page_order();
        Ok(s)
    }

    pub fn save(&self) -> Result<()> {
        let manifest_path = self.path.join("manifest.json");
        println!(
            "save path {:?}, manifest path {:?}",
            self.path, manifest_path
        );
        let json = serde_json::to_string_pretty(&self.manifest)?;
        save_persistent_data(manifest_path, &json);
        Ok(())
    }

    pub fn create_blank(path: PathBuf, name: String, color: Color32) -> Result<Self> {
        std::fs::create_dir_all(&path)?;

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
    pub fn get_path(&self) -> PathBuf {
        match self {
            FolderEntry::Folder(folder) => folder.path.clone(),
            FolderEntry::File(file) => file.path.clone(),
        }
    }
}
