use std::path::{Path, PathBuf};

use egui::Color32;

pub struct UserProject{
    path: PathBuf,
    color: Color32,
}

impl UserProject{
    pub fn new(path: PathBuf, color: Color32)->Self{
        Self{
            path,
            color, 
        }
    }
}
