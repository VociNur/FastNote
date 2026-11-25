use std::path::Path;
use std::rc::Rc;

use crate::model::fastnote_file::FastnoteFile;
use crate::model::fastnote_manifest::Manifest;
use crate::model::fastnote_page::FastnotePage;
use crate::model::fastnote_project::FastnoteProject;

//on dit que quelqu’un sait faire ça
pub trait Storage: std::fmt::Debug {
    fn save_manifest(&self, path: &Path, manifest: &Manifest) -> anyhow::Result<()>;
    fn load_manifest(&self, path: &Path) -> anyhow::Result<Manifest>;
    /// Crée un dossier (et ses parents). Utilisé par create_blank des pages.
    fn create_dirs(&self, path: &Path) -> anyhow::Result<()>;

    fn save_page(&self, page: &FastnotePage) -> anyhow::Result<()>;
    fn load_page(&self, path: &Path) -> anyhow::Result<()>;

    fn save_file(&self, file: &FastnoteFile) -> anyhow::Result<()>;
    fn load_file(&self, path: &Path) -> anyhow::Result<()>;

    fn save_project(&self, path: &FastnoteProject) -> anyhow::Result<()>;
    fn load_project(&self, path: &Path) -> anyhow::Result<()>;
}

pub type SharedStorage = Rc<dyn Storage>;
