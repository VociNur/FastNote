
use eframe::egui;
use egui::Context;
use serde::{Deserialize, Serialize};

use crate::{edition::open_edition_mode, gpuview::GpuView, pen::{DEFAULT_PEN, Pen}, themes::ThemeData, user_file::UserFile, user_project::UserProject};

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

    pub opened_projects: Vec<UserProject>,
    pub current_file: Option<UserFile>,

    pub pen: Pen,
    pub edition_open: bool,


    pub gpu_view: GpuView,    
}

impl Default for State{
    fn default() -> Self {
        Self{
            menu_mode: Menu::File,
            theme: ThemeData::default(),
            pen: DEFAULT_PEN,
            opened_projects: vec![],
            current_file: None,
            edition_open: false,
            gpu_view: GpuView::default(), 
        }
    }
}

impl State{
    pub fn new() -> Self{
        Self{
            menu_mode: Menu::File,
            theme: ThemeData::default(),
            // value: 3,
            pen: DEFAULT_PEN,
            opened_projects: vec![],
            current_file: None,
            edition_open: false,
            gpu_view: GpuView::default(), 
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
