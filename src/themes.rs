use egui::Color32;

use crate::str_hex_to_color;

enum Theme {
    White,
    Black,
    Violet,
}

pub struct ThemeData {
    pub menu_bg: Color32,
    pub menu_fg: Color32,
    pub menu_selected_bg: Color32,
    pub menu_selected_fg: Color32,

    
    pub ribbon_bg: Color32,
    pub ribbon_fg: Color32,
    pub ribbon_selected_bg: Color32,
    pub ribbon_selected_fg: Color32,
}


pub const DEFAULT_WHITE_THEME: ThemeData = ThemeData {
    menu_bg: Color32::from_rgb(240, 240, 240),
    menu_fg: Color32::BLACK,
    menu_selected_bg: Color32::from_rgb(120, 80, 255),
    menu_selected_fg: Color32::WHITE,

    ribbon_bg: Color32::from_rgb(240, 240, 240),
    ribbon_fg: Color32::BLACK,
    ribbon_selected_bg: Color32::from_rgb(120, 80, 255),
    ribbon_selected_fg: Color32::WHITE,
};

pub const DEFAULT_THEME: ThemeData = ThemeData {
    menu_bg: Color32::from_rgb(77, 79, 83),
    menu_fg: Color32::from_rgb(255, 255,255),
    menu_selected_bg: Color32::from_rgb(120, 80, 255),
    menu_selected_fg: Color32::WHITE,

    ribbon_bg: Color32::from_rgb(189, 189, 189),
    ribbon_fg: Color32::BLACK,
    ribbon_selected_bg: Color32::from_rgb(120, 80, 255),
    ribbon_selected_fg: Color32::WHITE,
};

// Ne sert à rien de faire si compliqué pour l'instant, peut être plus tard ?
// #[derive(serde::Deserialize)]
// pub struct RawTheme {
//     pub menu_bg: String,
//     pub menu_fg: String,
//     pub menu_selected_bg: String,
//     pub menu_selected_fg: String,
// }

// impl RawTheme {
//     pub fn to_theme_data(self) -> ThemeData {
//         ThemeData {
//             menu_bg: str_hex_to_color(&self.menu_bg),
//             menu_fg: str_hex_to_color(&self.menu_fg),
//             menu_selected_bg: str_hex_to_color(&self.menu_selected_bg),
//             menu_selected_fg: str_hex_to_color(&self.menu_selected_fg),
//         }
//     }
// }
