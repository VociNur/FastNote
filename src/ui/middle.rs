

use crate::{app::App, state::{Menu, State}, ui::{menu::draw_menu, ribbon::draw_ribbon}};

pub fn draw_middle(ui: &mut egui::Ui, app: &mut App){
    
    egui::CentralPanel::default().show_inside(ui, |ui| {//.frame(egui::Frame{fill: Color32::fromrgb(255, 0, 0), ..Default..default()})
        match app.state.get_menu(){
            Menu::File => draw_file_menu_middle(ui, app),
            _=>{}
        }
    });
}
pub fn draw_file_menu_middle(ui: &mut egui::Ui, app: &mut App){
            
    let open_folder_icon = egui::Image::new(&app.icons.open_folder)
        .fit_to_exact_size(egui::vec2(32.0, 32.0));
    let open_folder_button = ui.add_sized([32.0, 32.0], egui::Button::image(open_folder_icon).frame(false));
    if open_folder_button.clicked() {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            // path is a std::path::PathBuf
            app.open_project(path);
        }
    }
    if open_folder_button.hovered(){
        ui.painter().rect_filled(
            open_folder_button.rect,
            4.0,
            egui::Color32::from_white_alpha(100),
        );
    }
}
