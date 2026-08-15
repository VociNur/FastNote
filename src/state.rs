use std::path::PathBuf;

use eframe::egui;
use egui::Context;
use serde::{Deserialize, Serialize};

use crate::{
    edition::open_edition_mode,
    gpu::gpuview::GpuView,
    pen::color_palette::ColorPalette,
    projects::{
        fastnote_project::FastnoteFile, opened_projects::OpenedProjectsManager, user_file::UserFile,
    },
    themes::ThemeData,
    ui::modal_windows::modal_window::ModalWindow,
};

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
impl Default for MenuMode {
    fn default() -> Self {
        MenuMode::File
    }
}
impl MenuMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuMode::File => "File",
            MenuMode::Home => "Home",
            MenuMode::Insert => "Insert",
            MenuMode::Draw => "Draw",
            MenuMode::History => "History",
            MenuMode::View => "View",
            MenuMode::Edition => "Edition",
        }
    }
}

pub struct State {
    menu_mode: MenuMode,
    pub theme: ThemeData,

    //File
    pub modal_window: ModalWindow,
    pub opened_projects: OpenedProjectsManager,
    pub current_file: Option<UserFile>,
    pub current_fastnote_file: Option<PathBuf>,
    pub current_fastnote_page: Option<PathBuf>,
    //Menu
    // pub file_tree: FileTreeState,

    // pub pen: Pen,
    pub cursor_icon: egui::CursorIcon,
    pub color_palette: ColorPalette,
    pub edition_open: bool,

    pub gpu_view: GpuView,
}

impl Default for State {
    fn default() -> Self {
        let color_palette = ColorPalette::load().unwrap_or(ColorPalette::default());
        Self {
            menu_mode: MenuMode::File,
            theme: ThemeData::default(),
            color_palette: color_palette,
            // file_tree: FileTreeState::default(),
            modal_window: ModalWindow::None,
            cursor_icon: egui::CursorIcon::Default,
            opened_projects: OpenedProjectsManager::default(),
            current_file: None,
            current_fastnote_file: None,
            current_fastnote_page: None,
            edition_open: false,
            gpu_view: GpuView::default(),
        }
    }
}

impl State {
    pub fn set_menu(&mut self, menu: MenuMode, ctx: &Context) {
        if menu == MenuMode::Edition {
            self.edition_open = true;
            open_edition_mode(&mut self.theme, ctx, &mut self.edition_open);
        } else {
            self.menu_mode = menu
        }
    }
    pub fn get_menu(&self) -> MenuMode {
        self.menu_mode
    }
}
