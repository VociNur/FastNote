use std::path::Path;
use std::rc::Rc;
use crate::fastnote_project::Manifest;

pub trait Storage {
    fn save_manifest(&self, path: &Path, manifest: &Manifest) -> anyhow::Result<()>;
    fn load_manifest(&self, path: &Path) -> anyhow::Result<Manifest>;
    fn create_dirs(&self, path: &Path) -> anyhow::Result<()>;
}
pub type SharedStorage = Rc<dyn Storage>;
