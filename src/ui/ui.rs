use crate::{
    app::App, state::MenuMode, ui::{middle::{draw_file_menu_middle, draw_left}, top_bar::draw_top_bar, ui_gpu::draw_ui_gpu}
};
use eframe::egui::{self};

pub fn draw_gui(ui: &mut egui::Ui, app: &mut App) {
    draw_top_bar(ui, app);
    draw_left(ui, app);
    // println!("screen_rect: {:?}", ui.ctx().screen_rect());
    // println!("view_poirt: {:?}", ui.ctx().viewport_rect());
    if app.state.get_menu() != MenuMode::File{
        draw_ui_gpu(ui, app);
    }else{
        egui::CentralPanel::default().show_inside(ui, |ui| {
            draw_file_menu_middle(ui, app);
        });
    }
    // Fond grisé qui bloque les clics sur le reste
    //
    //
    if app.state.new_project_dialog.open {
        egui::Modal::new(egui::Id::new("new_project_modal")).show(ui, |ui| {
        ui.heading("New project");
    
        ui.label("Project name :");
        let response = ui.text_edit_singleline(&mut app.state.new_project_dialog.name);

        ui.label("Couleur :");
        ui.color_edit_button_srgba(&mut app.state.new_project_dialog.color);

        ui.separator();

        ui.horizontal(|ui: &mut egui::Ui| {
            let enter = response.lost_focus()
                && ui.input(|i| i.key_pressed(egui::Key::Enter));

            if ui.button("Créer").clicked() || enter {
                app.user_created_project();
                app.state.new_project_dialog.open = false;
            }
            if ui.button("Annuler").clicked() {
                app.state.new_project_dialog.open = false;
            }
        });
    });
    }
}
