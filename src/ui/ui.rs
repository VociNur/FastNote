use crate::{
    app::App,
    state::MenuMode,
    ui::{
        middle::{draw_file_menu_middle, draw_left},
        modal_windows::modal_window::draw_modal_window,
        top_bar::draw_top_bar,
        ui_gpu::draw_ui_gpu,
    },
};
use eframe::egui::{self};

pub fn draw_gui(ui: &mut egui::Ui, app: &mut App) {
    draw_top_bar(ui, app);
    draw_left(ui, app);
    // println!("screen_rect: {:?}", ui.ctx().screen_rect());
    // println!("view_poirt: {:?}", ui.ctx().viewport_rect());
    if app.state.get_menu() != MenuMode::File {
        draw_ui_gpu(ui, app);
    } else {
        egui::CentralPanel::default().show(ui, |ui| {
            draw_file_menu_middle(ui, app);
        });
    }
    // Fond grisé qui bloque les clics sur le reste
    //
    //
    //

    draw_modal_window(ui, app)
}

pub fn draw_error_banner(ui: &egui::Ui, message: &str, line: usize) {
    let ctx = ui.ctx();

    // Récupère le rectangle complet de la fenêtre
    let screen = ctx.input(|i| i.viewport_rect());

    let line_height = 32.0;
    let y_offset = line as f32 * line_height;

    egui::Area::new(egui::Id::new(format!("error_banner_{}", line)))
        .order(egui::Order::Foreground) // au-dessus de tout
        .fixed_pos(screen.min + egui::vec2(0.0, y_offset)) // position en haut + décalage
        .show(ctx, |ui| {
            let rect = egui::Rect {
                min: screen.min + egui::vec2(0.0, y_offset),
                max: screen.min + egui::vec2(screen.width(), y_offset + line_height),
            };

            // Fond rouge
            ui.painter()
                .rect_filled(rect, 0.0, egui::Color32::from_rgb(180, 20, 20));

            // Texte
            ui.painter().text(
                rect.min + egui::vec2(10.0, 8.0),
                egui::Align2::LEFT_TOP,
                message,
                egui::FontId::proportional(16.0),
                egui::Color32::WHITE,
            );
        });
}
