use eframe::egui;

use crate::{
    app::App,
    ui::modal_windows::{
        create_project_modal_window::{draw_new_project_modal_window, NewProjectModalWindow},
        new_file_modal_window::{draw_new_file_modal_window, NewFileModalWindow},
        new_folder_modal_window::{draw_new_folder_modal_window, NewFolderModalWindow},
        new_page_modal_window::{draw_new_page_modal_window, NewPageModalWindow},
        rename_modal_window::{draw_rename_modal_window, RenameModalWindow},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModalWindow {
    None,
    Taken,
    NewProject(NewProjectModalWindow),
    NewFolder(NewFolderModalWindow),
    NewFile(NewFileModalWindow),
    Rename(RenameModalWindow),
    NewPage(NewPageModalWindow),
}

impl ModalWindow {
    pub fn take(&mut self) -> ModalWindow {
        std::mem::replace(self, ModalWindow::Taken)
    }
}

pub fn draw_modal_window(ui: &egui::Ui, app: &mut App) {
    let mut modal_window = app.state.modal_window.take();
    // println!("modal w indow {:?}", modal_window);
    match modal_window {
        ModalWindow::None => {}
        ModalWindow::Taken => {}
        ModalWindow::NewProject(ref mut new_project_modal_window) => {
            let screen = ui.max_rect();

            ui.painter()
                .rect_filled(screen, 0.0, egui::Color32::from_black_alpha(50));

            draw_new_project_modal_window(ui, app, new_project_modal_window)
        }
        ModalWindow::NewFolder(ref mut new_folder_modal_window) => {
            let screen = ui.max_rect();

            ui.painter()
                .rect_filled(screen, 0.0, egui::Color32::from_black_alpha(50));
            draw_new_folder_modal_window(ui, app, new_folder_modal_window);
        }
        ModalWindow::NewFile(ref mut new_file_modal_window) => {
            let screen = ui.max_rect();

            ui.painter()
                .rect_filled(screen, 0.0, egui::Color32::from_black_alpha(50));
            draw_new_file_modal_window(ui, app, new_file_modal_window);
        }
        ModalWindow::Rename(ref mut modal) => {
            let screen = ui.max_rect();

            ui.painter()
                .rect_filled(screen, 0.0, egui::Color32::from_black_alpha(50));
            draw_rename_modal_window(ui, app, modal);
        }
        ModalWindow::NewPage(ref mut modal) => {
            let screen = ui.max_rect();

            ui.painter()
                .rect_filled(screen, 0.0, egui::Color32::from_black_alpha(50));
            draw_new_page_modal_window(ui, app, modal);
        }
    }
    if app.state.modal_window == ModalWindow::Taken {
        app.state.modal_window = modal_window;
    }
}
