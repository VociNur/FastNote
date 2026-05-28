use eframe::egui;
use std::path::PathBuf;

use egui::Color32;
use serde::{Deserialize, Serialize};

use crate::{paths::MAIN_DATA, save_persistent_data};

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct UserProject {
    pub path: PathBuf,//here is a folder !
    name: String,
    color: Color32,
}

impl UserProject {
    pub fn create_blank_project(path: PathBuf, name: String, color: Color32) -> Self {
        let s = Self { path:path, name, color };
        let err_save = s.save();
        if err_save.is_err(){
            println!("An error while saving the file: {:?}", s.path);
            println!("{:?}", err_save.err());
        }
        s
    }

    //     pub fn from_path(path: PathBuf){
    //         //path here is a folder !
    //         //
    //         // load_persistent_data(path)
    //     }
    pub fn save(&self) -> anyhow::Result<()>{

        let json = serde_json::to_string_pretty(self)?;
        // println!("json {:?} ", json);
        save_persistent_data(self.path.join(MAIN_DATA), &json);
        Ok(())
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_color(&self) -> &Color32 {
        &self.color
    }

    pub fn set_name(&mut self, name: String){
        self.name = name;
        let err = self.save();
        if err.is_err(){
            println!("Could not save project ! {:?}", self.path)
        }
    }

    pub fn set_color(&mut self, color: Color32){
        self.color = color;
        let err = self.save();
        if err.is_err(){
            println!("Could not save project ! {:?}", self.path)
        }
    }

    
}
