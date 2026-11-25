//! Adapter disque : la seule implémentation de `Storage` qui touche au système de fichiers.

use anyhow::Result;
use fastnote_domain::model::{fastnote_manifest::Manifest, fastnote_project::FastnoteProject};
use fastnote_domain::storage::Storage;
use std::path::{Path, PathBuf};

use crate::{get_working_path, paths::OPENED_PROJECTS_FILE};

pub struct DiskStorage;

impl DiskStorage {
    fn new() -> DiskStorage {
        DiskStorage {}
    }
}

impl DiskStorage {
    fn get_opened_projects_file_path() -> PathBuf {
        get_working_path().join(OPENED_PROJECTS_FILE)
    }

    fn load_linux_folder(&self, path: PathBuf) {
        let manifest_path = path.join("manifest.json");
        let json = load_persistent_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;
        // if manifest.item_type
        todo!()
    }
}

impl Storage for DiskStorage {
    fn save_manifest_file(&self, path: &Path, manifest: &Manifest) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(manifest)?;
        crate::persist::my_save_json_data(path, &json)
    }

    fn load_manifest_file(&self, path: &Path) -> anyhow::Result<Manifest> {
        let json = crate::persist::my_load_json(path)?;
        Ok(serde_json::from_str(&json))
    }

    fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(path);
        Ok(())
    }

    fn save_region_file(
        &self,
        region: &fastnote_domain::model::region::LoadedRegion,
    ) -> anyhow::Result<()> {
        todo!()

        // let region_path = regions_path.join(self.filename());
        // let json = serde_json::to_string_pretty(&self)?;
        // save_persistent_data(region_path, &json);
        // Ok(())
    }

    fn load_region_file(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::region::LoadedRegion> {
        todo!()
    }

    // celle ci doivent vraiment être implémentée ?
    // j’ai envie de dire que oui
    fn save_page_file(
        &self,
        page: &fastnote_domain::model::fastnote_page::FastnotePage,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn load_page_file(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_page::FastnotePage> {
        todo!()
    }

    fn save_file_file(
        &self,
        file: &fastnote_domain::model::fastnote_file::FastnoteFile,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn load_file_file(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_file::FastnoteFile> {
        todo!()
    }

    fn save_project_file(
        &self,
        project: &fastnote_domain::model::fastnote_project::FastnoteProject,
    ) -> anyhow::Result<()> {
        todo!()
    }
    fn load_project(
        &self,
        path: PathBuf,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_project::FastnoteProject> {
        let manifest_path = path.join("manifest.json");
        let json = load_persistent_data(manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&json)?;

        let mut children = vec![];
        for entry in std::fs::read_dir(&path)? {
            let entry = entry?;
            let child_path = entry.path();

            // On ne charge que les dossiers contenant un manifest.json
            if child_path.is_dir() && child_path.join("manifest.json").exists() {
                // children.push(FolderEntry::load(storage.clone(), child_path)?);
                children.push(self.storage.load_entry())
            }
        }

        let project: FastnoteProject = FastnoteProject {
            path: path,
            manifest,
            children,
            storage,
        };
        Ok(project)
    }

    fn save_open_projects_file(&self, path_list: &[PathBuf]) -> anyhow::Result<()> {
        todo!()
    }

    fn load_open_projects_file(&self) -> anyhow::Result<Vec<PathBuf>> {
        let path_opened_projects_data = self.get_opened_projects_file_path();
        let json = load_persistent_data(path_opened_projects_data)?;
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
}
