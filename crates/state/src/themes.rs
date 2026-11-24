use eframe::egui;
use egui::Color32;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct ThemeData {
    pub menu_bg: Color32,
    pub menu_fg: Color32,
    pub menu_selected_bg: Color32,
    pub menu_selected_fg: Color32,
    pub menu_size_text: f32,
    pub space_between_menu_and_ribbon: f32,
    

    
    pub ribbon_bg: Color32,
    pub ribbon_fg: Color32,
    pub ribbon_selected_bg: Color32,
    pub ribbon_selected_fg: Color32,
    pub ribbon_size: f32,

    pub notebook_tree_text_file_fg: Color32,
    pub notebook_tree_text_file_bg: Color32,
    pub notebook_tree_text_folder_fg: Color32,
    pub notebook_tree_text_folder_bg: Color32,
    pub notebook_tree_text_size: f32,
}

// pub const DEFAULT_WHITE_THEME: ThemeData = ThemeData {
//     menu_bg: Color32::from_rgb(240, 240, 240),
//     menu_fg: Color32::BLACK,
//     menu_selected_bg: Color32::from_rgb(120, 80, 255),
//     menu_selected_fg: Color32::WHITE,

//     ribbon_bg: Color32::from_rgb(240, 240, 240),
//     ribbon_fg: Color32::BLACK,
//     ribbon_selected_bg: Color32::from_rgb(120, 80, 255),
//     ribbon_selected_fg: Color32::WHITE,
// };
impl Default for ThemeData{
    fn default() -> Self {
        
        ThemeData {
            //MENU
            menu_bg: Color32::from_rgb(77, 79, 83),
            menu_fg: Color32::from_rgb(255, 255,255),
            menu_size_text: 18.,
            // menu_rel_size_text: 0., osef en vrai
            space_between_menu_and_ribbon: 16.,
            menu_selected_bg: Color32::from_rgb(120, 80, 255),
            menu_selected_fg: Color32::WHITE,
    
            //RIBBON
            ribbon_bg: Color32::from_rgb(189, 189, 189),
            ribbon_fg: Color32::BLACK,
            ribbon_selected_bg: Color32::from_rgb(120, 80, 255),
            ribbon_selected_fg: Color32::WHITE,
            ribbon_size: 18.,

            //FILE

            //HOME
            //TREE
            notebook_tree_text_file_fg: Color32::BLACK,
            notebook_tree_text_file_bg: Color32::from_rgba_unmultiplied(0, 255, 255, 255),

            notebook_tree_text_folder_fg: Color32::BLACK,
            notebook_tree_text_folder_bg: Color32::from_rgba_unmultiplied(0, 255, 255, 255),
            notebook_tree_text_size: 18.,
        }
    
    }
}



