use eframe::egui;
use egui::Context;
use serde::{Deserialize, Serialize};

use crate::{edition::open_edition_mode, pen::{DEFAULT_PEN, Pen}, themes::ThemeData, user_project::UserProject};

#[derive(PartialEq, PartialOrd, Clone, Copy, Deserialize, Serialize)]
pub enum Menu {
    File,
    Home,
    Insert,
    Draw,
    History,
    View,
    Edition,
}

impl Menu {
    pub fn as_str(&self) -> &'static str{
        match self {
            Menu::File => "File",
            Menu::Home => "Home",
            Menu::Insert => "Insert",
            Menu::Draw => "Draw",
            Menu::History => "History",
            Menu::View => "View",
            Menu::Edition => "Edition"
        }
    }
}


// #[derive(Serialize, Deserialize, Debug)]
// 
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct State{
    menu_mode: Menu,
    pub theme: ThemeData,
    pub value: String,
    pub pen: Pen,
    pub opened_projects: Vec<UserProject>,
    pub ajout: String,
    pub edition_open: bool,
    
}

impl Default for State{
    fn default() -> Self {
        Self{
            menu_mode: Menu::File,
            theme: ThemeData::default(),
            value: "".to_owned(),
            pen: DEFAULT_PEN,
            opened_projects: vec![],
            ajout: "auie".to_owned(),
            edition_open: false,
        }
    }
}

impl State{
    pub fn new() -> Self{
        Self{
            menu_mode: Menu::File,
            theme: ThemeData::default(),
            // value: 3,
            value: "Oh".to_owned(),
            pen: DEFAULT_PEN,
            opened_projects: vec![],
            ajout: "auie".to_owned(),
            edition_open: false,
        }
    }
    pub fn set_menu(&mut self, menu: Menu, ctx: &Context){
        if menu == Menu::Edition {
            self.edition_open = true;
            open_edition_mode(&mut self.theme, ctx, &mut self.edition_open);
        }else{
            
            self.menu_mode = menu
        }
    }
    pub fn get_menu(&self) -> Menu{
        self.menu_mode
    }

}
