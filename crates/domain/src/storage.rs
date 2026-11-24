use crate::fastnote_project::Manifest;
use std::path::Path;
use std::rc::Rc;

//on dit que quelqu’un sait faire ça
pub trait Storage: std::fmt::Debug {
    fn save_manifest(&self, path: &Path, manifest: &Manifest) -> anyhow::Result<()>;
    fn load_manifest(&self, path: &Path) -> anyhow::Result<Manifest>;
    /// Crée un dossier (et ses parents). Utilisé par create_blank des pages.
    fn create_dirs(&self, path: &Path) -> anyhow::Result<()>;
}

pub type SharedStorage = Rc<dyn Storage>;
