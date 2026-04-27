
use crate::{pen::{DEFAULT_PEN, Pen}, themes::{DEFAULT_THEME, ThemeData}, user_project::UserProject};

#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum Menu {
    File,
    Home,
    Insert,
    Draw,
    History,
    View,
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
        }
    }
}



pub struct State{
    menu_mode: Menu,
    pub theme: ThemeData,
    pub value: String,
    pub pen: Pen,
    pub opened_projects: Vec<UserProject>,
    
    
}

impl State{
    pub fn new() -> Self{
        Self{
            menu_mode: Menu::File,
            theme: DEFAULT_THEME,
            // value: 3,
            value: "Oh".to_owned(),
            pen: DEFAULT_PEN,
            opened_projects: vec![],
        }
    }
    pub fn set_menu(&mut self, menu: Menu){
        self.menu_mode = menu
    }
    pub fn get_menu(&self) -> Menu{
        self.menu_mode
    }

}
