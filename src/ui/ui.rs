use crate::{
    app::App, gpu::first_line::StrokeCallback, ui::{middle::draw_middle, top_bar::draw_top_bar}
};
use eframe::egui::{self, Pos2, Rect};

pub fn draw_gui(ui: &mut egui::Ui, app: &mut App) {
    draw_top_bar(ui, app);
    draw_middle(ui, app);
    // println!("screen_rect: {:?}", ui.ctx().screen_rect());
    // println!("view_poirt: {:?}", ui.ctx().viewport_rect());

    if app.state.current_file.is_some() {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            let rect = ui.available_rect_before_wrap();
            app.gpu_rect = Some(rect.clone());
            // println!("Rect : {}", rect);
            // let adj_rect = Rect {min: rect.min, max: Pos2 {x: rect.max.x, y: 1080f32}};
            // println!("Rect : {}", adj_rect);

            let points = app
                .state
                .current_file
                .as_ref()
                .map(|f| f.current_stroke.clone())
                .unwrap_or_default();
            ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);
            let current_stroke = app.state.current_file
            .as_ref()
            .map(|f| f.current_stroke.clone())
            .unwrap_or_default();

            let strokes = app.state.current_file
                .as_ref()
                .map(|f| f.strokes.clone())
                .unwrap_or_default();

            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                rect,
                StrokeCallback {
                    current_stroke,
                    strokes,
                    canvas_size: rect.size(),
                },
            ));
        });
    }
}
