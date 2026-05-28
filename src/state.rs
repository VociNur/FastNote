
use eframe::egui;
use egui::Context;
use serde::{Deserialize, Serialize};

use crate::{edition::open_edition_mode, file_tree_state::FileTreeState, get_working_path, gpuview::GpuView, has_persisent_data, load_persistent_data, paths::OPENED_PROJECTS_FILE, pen::{DEFAULT_PEN, Pen}, projects::{opened_projects::OpenedProjectsManager, user_file::UserFile, user_project::UserProject}, themes::ThemeData, ui::create_project_modal_window::NewProjectDialog};

#[derive(PartialEq, PartialOrd, Clone, Copy, Deserialize, Serialize)]
pub enum MenuMode {
    File,
    Home,
    Insert,
    Draw,
    History,
    View,
    Edition,
}

impl MenuMode {
    pub fn as_str(&self) -> &'static str{
        match self {
            MenuMode::File => "File",
            MenuMode::Home => "Home",
            MenuMode::Insert => "Insert",
            MenuMode::Draw => "Draw",
            MenuMode::History => "History",
            MenuMode::View => "View",
            MenuMode::Edition => "Edition"
        }
    }
}


// #[derive(Serialize, Deserialize, Debug)]
// 
// #[derive(Serialize, Deserialize)]
// #[serde(default)]
pub struct State{
    menu_mode: MenuMode,
    pub theme: ThemeData,

    //File
    pub new_project_dialog: NewProjectDialog, 
    pub opened_projects: OpenedProjectsManager,
    pub current_file: Option<UserFile>,

    //Menu
    pub file_tree: FileTreeState,

    pub pen: Pen,
    pub cursor_icon: egui::CursorIcon,
    pub edition_open: bool,


    pub gpu_view: GpuView,    
}

impl Default for State{
    fn default() -> Self {
        
        Self{
            menu_mode: MenuMode::File,
            theme: ThemeData::default(),
            pen: DEFAULT_PEN,
            file_tree: FileTreeState::default(),
            new_project_dialog: NewProjectDialog::default(), 
            cursor_icon: egui::CursorIcon::Default,
            opened_projects: OpenedProjectsManager::default(),
            current_file: None,
            edition_open: false,
            gpu_view: GpuView::default(), 
        }
    }
}

impl State{
    pub fn new() -> Self{
        Self{
            menu_mode: MenuMode::File,
            theme: ThemeData::default(),
            file_tree: FileTreeState::default(),
            // value: 3,
            pen: DEFAULT_PEN,
            new_project_dialog: NewProjectDialog::default(), 
            cursor_icon: egui::CursorIcon::Default,
            opened_projects: OpenedProjectsManager::default(),
            current_file: None,
            edition_open: false,
            gpu_view: GpuView::default(), 
        }
    }
    pub fn set_menu(&mut self, menu: MenuMode, ctx: &Context){
        if menu == MenuMode::Edition {
            self.edition_open = true;
            open_edition_mode(&mut self.theme, ctx, &mut self.edition_open);
        }else{
            
            self.menu_mode = menu
        }
    }
    pub fn get_menu(&self) -> MenuMode{
        self.menu_mode
    }
}
