use crate::{
    app::App, gpu::first_line::StrokeCallback, ui::{middle::draw_left, top_bar::draw_top_bar}
};
use eframe::egui::{self, Pos2, Rect};

pub fn draw_gui(ui: &mut egui::Ui, app: &mut App) {
    draw_top_bar(ui, app);
    draw_left(ui, app);
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

        let painter = ui.painter();
        let zoom = app.state.gpu_view.zoom;
        let offset = app.state.gpu_view.top_left; // le décalage actuel

        // Lignes horizontales tous les 50 pixels (dans l'espace canvas)
        let line_spacing = 50.0 * zoom;
        let start_y = rect.min.y - (offset.y * zoom) % line_spacing;
        let mut y = start_y;
        while y < rect.max.y {
            painter.line_segment(
                [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                egui::Stroke::new(1.0, egui::Color32::from_rgb(200, 210, 255)),
            );
            y += line_spacing;
        }

        // Marge verticale rouge
        let margin_x = rect.min.x + 80.0 * zoom - offset.x * zoom;
        if margin_x > rect.min.x && margin_x < rect.max.x {
            painter.line_segment(
                [egui::pos2(margin_x, rect.min.y), egui::pos2(margin_x, rect.max.y)],
                egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 100, 100)),
            );
        }

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
                    gpu_view: app.state.gpu_view.clone(),
                },
            ));
        });
    }
}
