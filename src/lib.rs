// #![warn(clippy::all, rust_2018_idioms)]

use std::path::{self, PathBuf};

use eframe::egui;

use crate::paths::PERSISTENT;

// use egui::Color32;
pub mod app;
pub mod state;
pub mod themes;
pub mod ui;
pub mod icons;
pub mod pen;
pub mod projects;
pub mod edition;
pub mod stylet;
pub mod gpu;
pub mod strokes;
pub mod input_manager;
pub mod gpuview;
pub mod paths;
pub mod file_tree_state;
pub mod tree_order;

// pub fn str_hex_to_color(hex: &str)->Color32{
//     let hx = hex.trim_start_matches("#");
//     let r = u8::from_str_radix(&hx[0..2],16).unwrap();
//     let g = u8::from_str_radix(&hx[2..4],16).unwrap();
//     let b = u8::from_str_radix(&hx[4..6],16).unwrap();
//     Color32::from_rgb(r, g, b)
// }
fn get_screen_size() -> (u32, u32) {
    let Ok(output) = std::process::Command::new("xrandr").output() else {
        return (1920, 1200);
    };
    
    let text = String::from_utf8_lossy(&output.stdout);
    for line in text.lines() {
        if line.contains(" connected primary") {
            if let Some(res) = line.split_whitespace()
                .find(|s| s.contains('x') && s.contains('+'))
            {
                let parts: Vec<&str> = res.split('x').collect();
                if parts.len() >= 2 {
                    let w = parts[0].parse().unwrap_or(1920);
                    let h = parts[1].split('+').next()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(1200);
                    return (w, h);
                }
            }
        }
    }
    println!("Error !");
    assert!(false);
    (1,1)
}

fn get_working_path() -> std::path::PathBuf{
    dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
    
}

// fn get_path_with_name(name: String) -> std::path::PathBuf {
//     let path = dirs::data_dir()                    // C:\Users\...\AppData\Roaming  (Windows)
//                                         // ~/.local/share               (Linux)
//                                         // ~/Library/Application Support (Mac)
//         .unwrap_or_else(|| std::path::PathBuf::from("."))
//         .join("fastnote")
//         .join("persistent")
//         .join(format!("{name}.json"));
//     println!("Path : {:?} {:?}", &path, path::absolute(&path));
//     path
// }


// fn save_persistent_data_file(filename: &str, json: &str) -> anyhow::Result<()> {
//         let path = get_path_with_name(filename.to_owned());
    
//         let tmp_path = path.with_extension("tmp");

//          if let Some(parent) = path.parent() {
//             std::fs::create_dir_all(parent)?;
//         }
//         std::fs::write(&tmp_path, json)?;
    
//         // 2. Renommer atomiquement → remplace l'ancien fichier d'un coup
//         std::fs::rename(&tmp_path, path)?;
        
//         Ok(())
// }

fn aux_save_persistent_data(path:PathBuf, json: &str) -> anyhow::Result<()>{
    
        let tmp_path = path.with_extension("tmp");

         if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&tmp_path, json)?;
    
        // 2. Renommer atomiquement → remplace l'ancien fichier d'un coup
        std::fs::rename(&tmp_path, path)?;
        
        Ok(())
    
}

fn save_persistent_data(path: PathBuf, json: &str){
    if aux_save_persistent_data(path.clone(), json).is_err(){
        println!("Could not save file: {path:?}");
    }
}

fn has_persisent_data(path: PathBuf)->bool{
    
        // let path = get_path_with_name(filename.to_owned());
        path.exists()
}



fn load_persistent_data(path: PathBuf) -> anyhow::Result<String> {
        // let path = get_path_with_name(filename.to_owned());
    
        
        if path.exists() {
            let json = std::fs::read_to_string(path).unwrap_or_default();
            Ok(json)
        } else {
           // State::default()
           println!("Opening data that doesn't exist");
           println!("File {:?}", path);
           Err(anyhow::anyhow!("File not found"))
        }

}
fn distance_point_to_segment(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> f32 {
    let ab = b - a;
    let ap = p - a;
    let t  = (ap.dot(ab) / ab.dot(ab)).clamp(0.0, 1.0);
    let closest = a + ab * t;
    (p - closest).length()
}
