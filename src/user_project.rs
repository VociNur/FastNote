use eframe::egui;
use std::path::PathBuf;

use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Clone,  Serialize, Deserialize)]
pub struct UserProject{
    pub name: String,
    pub path: PathBuf,
    pub color: Color32,
}

// impl Default for UserProject{
    
// }

impl UserProject{
    pub fn new(name: String, path: PathBuf, color: Color32)->Self{
        Self{
            name,
            path,
            color, 
        }
    }
}
