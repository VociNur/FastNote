//! Adapter disque : la seule implémentation de `Storage` qui touche au système de fichiers.

use anyhow::Result;
use fastnote_domain::model::fastnote_file::FastnoteFile;
use fastnote_domain::model::fastnote_folder::FastnoteFolder;
use fastnote_domain::model::fastnote_folderentry::FolderEntry;
use fastnote_domain::model::fastnote_page::FastnotePage;
use fastnote_domain::model::fastnote_project::ItemType;
use fastnote_domain::model::{fastnote_manifest::Manifest, fastnote_project::FastnoteProject};
use fastnote_domain::storage::Storage;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::persist::my_load_json_data;
use crate::{get_working_path, paths::OPENED_PROJECTS_FILE};

#[derive(Debug, Clone, Copy)]
pub struct DiskStorage;

impl DiskStorage {
    fn new() -> DiskStorage {
        DiskStorage {}
    }
}

impl DiskStorage {
    fn get_opened_projects_file_path(&self) -> PathBuf {
        get_working_path().join(OPENED_PROJECTS_FILE)
    }

    fn load_linux_folder(&self, path: PathBuf) -> Result<String> {
        let manifest_path = path.join("manifest.json");
        let json = my_load_json_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;
        if manifest.r#type == fastnote_domain::model::fastnote_project::ItemType::Folder {}
        todo!()
    }
}

impl Storage for DiskStorage {
    fn save_manifest_file(&self, path: &Path, manifest: &Manifest) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(manifest)?;
        crate::persist::my_save_json_data(path, &json)
    }

    fn load_manifest_file(&self, path: &Path) -> anyhow::Result<Manifest> {
        let json = crate::persist::my_load_json_data(path)?;
        Ok(serde_json::from_str(&json)?)
    }

    fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(path);
        Ok(())
    }

    // celle ci doivent vraiment être implémentée ?
    // j’ai envie de dire que oui
    fn save_page_file(
        &self,
        page: &fastnote_domain::model::fastnote_page::FastnotePage,
    ) -> anyhow::Result<()> {
        let page_path = &page.path;
        fs::create_dir_all(page_path.clone())?;
        // 2. Créer les sous-dossiers obligatoires
        fs::create_dir_all(page_path.join("regions"))?;

        fs::create_dir_all(page_path.join("assets"))?;
        let manifest_path = page_path.join("manifest.json");
        self.save_manifest_file(&manifest_path, &page.manifest)
    }

    fn load_page_file(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_page::FastnotePage> {
        let manifest_path = path.join("manifest.json");
        let manifest = self.load_manifest_file(&manifest_path)?;
        Ok(FastnotePage {
            path: path.to_path_buf(),
            manifest,
            storage: Rc::new(*self),
        })
    }

    fn save_file_file(
        &self,
        file: &fastnote_domain::model::fastnote_file::FastnoteFile,
    ) -> anyhow::Result<()> {
        self.save_manifest_file(&file.path.join("manifest.json"), &file.manifest)
    }

    fn load_file_file(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_file::FastnoteFile> {
        let manifest_path = path.join("manifest.json");
        let manifest = self.load_manifest_file(&manifest_path)?;

        let mut children = vec![];
        // Parcourir les sous-dossiers du fichier
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();
            // On ne charge que les dossiers contenant un manifest.json
            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                // Lire le manifest du dossier enfant
                let child_manifest = self.load_manifest_file(&child_path.join("manifest.json"))?;

                match child_manifest.r#type {
                    ItemType::Page => {
                        // Vérifier la structure obligatoire d'une Page
                        let regions = child_path.join("regions");
                        let assets = child_path.join("assets");

                        if !regions.exists() || !assets.exists() {
                            // Page corrompue → on ne la charge pas
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
                        // let page = FastnotePage::load(sm.clone(), child_path.clone())?;
                        let page = self.load_page_file(&child_path)?;
                        children.push(page);
                    }

                    other => {
                        // Un File ne doit contenir que des Pages
                        println!("File {:?} contains invalid child type {:?}", path, other);
                    }
                }
            }
        }

        let mut s = FastnoteFile {
            path: path.to_path_buf(),
            manifest,
            children,
            storage: Rc::new(*self),
        };
        s.update_page_order();
        Ok(s)
    }

    fn save_project_file(
        &self,
        project: &fastnote_domain::model::fastnote_project::FastnoteProject,
    ) -> anyhow::Result<()> {
        let manifest_path = project.path.join("manifest.json");
        self.save_manifest_file(&manifest_path, &project.manifest)
    }
    fn load_project(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_project::FastnoteProject> {
        let manifest_path = path.join("manifest.json");
        let json = my_load_json_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;

        let mut children = vec![];
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();

            // On ne charge que les dossiers contenant un manifest.json
            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                // children.push(FolderEntry::load(storage.clone(), child_path)?);
                children.push(self.load_folderentry_file(&child_path)?)
            }
        }

        let project: FastnoteProject = FastnoteProject {
            path: path.to_path_buf(),
            manifest,
            children,
            storage: Rc::new(*self),
        };
        Ok(project)
    }

    fn save_open_projects_file(&self, path_list: &[PathBuf]) -> anyhow::Result<()> {
        todo!()
    }

    fn load_open_projects_file(&self) -> anyhow::Result<Vec<PathBuf>> {
        let path_opened_projects_data = self.get_opened_projects_file_path();
        let json = my_load_json_data(path_opened_projects_data)?;
        let cast: Vec<PathBuf> = serde_json::from_str(&json)?;
        Ok(cast)
    }

    fn save_color_palette_file(
        &self,
        color_palette: &fastnote_domain::model::pen::color_palette::ColorPalette,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn load_color_palette_file(
        &self,
    ) -> anyhow::Result<fastnote_domain::model::pen::color_palette::ColorPalette> {
        todo!()
    }

    fn save_region_file(
        &self,
        path: &Path,
        region: &fastnote_domain::model::region::LoadedRegion,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn load_region_file(
        &self,
        regions_path: &Path,
        x: i32,
        y: i32,
    ) -> anyhow::Result<fastnote_domain::model::region::LoadedRegion> {
        todo!()
    }

    fn load_folderentry_file(&self, path: &Path) -> anyhow::Result<FolderEntry> {
        // Charger le manifest du dossier/fichier
        let manifest_path = path.join("manifest.json");
        let manifest = self.load_manifest_file(&manifest_path)?;

        match manifest.r#type {
            ItemType::Folder => Ok(FolderEntry::Folder(self.load_folder_file(&path)?)),
            ItemType::File => Ok(FolderEntry::File(self.load_file_file(&path)?)),
            _ => Err(anyhow::anyhow!(
                "FolderEntry::load: type {:?} invalide pour un enfant",
                manifest.r#type
            )),
        }
    }

    fn save_folder_file(&self, file: &FastnoteFolder) -> anyhow::Result<()> {
        self.save_manifest_file(&file.path.join("manifest.json"), &file.manifest)
    }

    fn load_folder_file(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_folder::FastnoteFolder> {
        let manifest_path = path.join("manifest.json");
        let manifest = self.load_manifest_file(&manifest_path)?;

        let mut children = vec![];

        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();

            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                // children.push(FolderEntry::load(storage.clone(), child_path)?);
                let child = self.load_folderentry_file(&child_path)?;
                children.push(child);
            }
        }

        let mut s = FastnoteFolder {
            path: path.to_path_buf(),
            manifest,
            children,
            storage: Rc::new(*self),
        };
        s.update_page_order();
        Ok(s)
    }
}
