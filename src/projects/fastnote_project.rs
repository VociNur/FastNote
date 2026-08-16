use std::{fs, path::PathBuf};

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
    pub name: String,
    pub color: egui::Color32,
    pub is_open: bool, //osef pour un file/autre
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

        Ok(Self {
            path,
            manifest,
            children,
        })
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

        Ok(Self {
            path,
            manifest,
            children,
        })
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
        };

        let folder = Self {
            path,
            manifest,
            children: vec![],
        };
        folder.save()?;
        Ok(folder)
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
        Ok(Self {
            path,
            manifest,
            children,
        })
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
        };

        let file = Self {
            path,
            manifest,
            children: vec![],
        };
        file.save()?;
        Ok(file)
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
}
