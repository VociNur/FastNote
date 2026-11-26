use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::model::fastnote_file::FastnoteFile;
use crate::model::fastnote_folder::FastnoteFolder;
use crate::model::fastnote_folderentry::FolderEntry;
use crate::model::fastnote_manifest::Manifest;
use crate::model::fastnote_page::FastnotePage;
use crate::model::fastnote_project::FastnoteProject;
use crate::model::pen::color_palette::ColorPalette;
use crate::model::region::LoadedRegion;

//on dit que quelqu’un sait faire ça
pub trait Storage: std::fmt::Debug {
    fn create_dirs(&self, path: &Path) -> anyhow::Result<()>;

    fn save_manifest_file(&self, path: &Path, manifest: &Manifest) -> anyhow::Result<()>;
    fn load_manifest_file(&self, path: &Path) -> anyhow::Result<Manifest>;

    fn save_region_file(&self, path: &Path, region: &LoadedRegion) -> anyhow::Result<()>;
    fn load_region_file(&self, regions_path: &Path, x: i32, y: i32)
        -> anyhow::Result<LoadedRegion>;

    fn save_page_file(&self, page: &FastnotePage) -> anyhow::Result<()>;
    fn load_page_file(&self, path: &Path) -> anyhow::Result<FastnotePage>;

    fn save_file_file(&self, file: &FastnoteFile) -> anyhow::Result<()>;
    fn load_file_file(&self, path: &Path) -> anyhow::Result<FastnoteFile>;

    fn save_folder_file(&self, file: &FastnoteFolder) -> anyhow::Result<()>;
    fn load_folder_file(&self, path: &Path) -> anyhow::Result<FastnoteFolder>;

    fn save_project_file(&self, project: &FastnoteProject) -> anyhow::Result<()>;
    fn load_project(&self, path: &Path) -> anyhow::Result<FastnoteProject>;

    fn load_folderentry_file(&self, path: &Path) -> anyhow::Result<FolderEntry>;
    // fn save_pen(&self, path: &FastnoteProject) -> anyhow::Result<()>;
    // fn load_pen(&self, path: &Path) -> anyhow::Result<()>;

    fn save_color_palette_file(&self, color_palette: &ColorPalette) -> anyhow::Result<()>;
    fn load_color_palette_file(&self) -> anyhow::Result<ColorPalette>;

    fn load_open_projects_file(&self) -> anyhow::Result<Vec<PathBuf>>;
    fn save_open_projects_file(&self, path_list: &[PathBuf]) -> anyhow::Result<()>;
}

pub type SharedStorage = Rc<dyn Storage>;
