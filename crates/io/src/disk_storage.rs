//! Adapter disque : la seule implémentation de `Storage` qui touche au système de fichiers.

use std::path::Path;

use fastnote_domain::fastnote_project::Manifest;
use fastnote_domain::storage::Storage;

pub struct DiskStorage;

impl Storage for DiskStorage {
    fn save_manifest(&self, path: &Path, manifest: &Manifest) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(manifest)?;
        crate::persist::save_json_atomic(path, &json)
    }

    fn load_manifest(&self, path: &Path) -> anyhow::Result<Manifest> {
        let json = crate::persist::load_json(path)?;
        Ok(serde_json::from_str(&json)?)
    }

    fn create_dirs(&self, path: &Path) -> anyhow::Result<()> {
        std::fs::create_dir_all(path)?;
        Ok(())
    }

    fn save_page(
        &self,
        page: &fastnote_domain::model::fastnote_page::FastnotePage,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn load_page(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_page::FastnotePage> {
        todo!()
    }

    fn save_file(
        &self,
        file: &fastnote_domain::model::fastnote_file::FastnoteFile,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn load_file(
        &self,
        path: &Path,
    ) -> anyhow::Result<fastnote_domain::model::fastnote_file::FastnoteFile> {
        todo!()
    }

    fn save_project(
        &self,
        path: &fastnote_domain::model::fastnote_project::FastnoteProject,
    ) -> anyhow::Result<()> {
        todo!()
    }

    fn load_project(&self, path: &Path) -> anyhow::Result<()> {
        todo!()
    }
}
