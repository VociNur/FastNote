use eframe::egui::{self, Color32};

use crate::{app::App, gpu::main_gpu::draw_gpu};
fn rect_points_to_pixels(rect_points: egui::Rect, ppp: f32) -> egui::Rect {
    egui::Rect::from_min_max(rect_points.min * ppp, rect_points.max * ppp)
}

pub fn draw_ui_gpu(ui: &mut egui::Ui, app: &mut App) {
    if app.state.current_file.is_some() {
        egui::CentralPanel::default().show(ui, |ui| {
            let rect = ui.available_rect_before_wrap();
            let pixel_rect = rect_points_to_pixels(rect, ui.pixels_per_point());
            app.gpu_rect = Some(pixel_rect);

            ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);
            let painter = ui.painter();

            let zoom = app.state.gpu_view.zoom;
            let offset = app.state.gpu_view.top_left; // le décalage actuel

            // Lignes horizontales tous les 50 pixels (dans l'espace canvas)
            let line_spacing = 50.0 * zoom / ui.pixels_per_point();
            let start_y = rect.min.y - (offset.y * zoom / ui.pixels_per_point()) % line_spacing;
            let mut y = start_y;
            while y < rect.max.y {
                painter.line_segment(
                    [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                    egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(200, 210, 255)),
                );
                y += line_spacing / ui.pixels_per_point();
            }

            // Marge verticale rouge
            let margin_x = rect.min.x + 80.0 * zoom - offset.x * zoom / ui.pixels_per_point();
            if margin_x > rect.min.x && margin_x < rect.max.x {
                painter.line_segment(
                    [
                        egui::pos2(margin_x, rect.min.y),
                        egui::pos2(margin_x, rect.max.y),
                    ],
                    egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(255, 100, 100)),
                );
            }
            // chunks
            let line_spacing = 1000.0 * zoom;
            let start_y = rect.min.y - (offset.y * zoom / ui.pixels_per_point()) % line_spacing;
            let mut y = start_y;
            while y < rect.max.y {
                painter.line_segment(
                    [egui::pos2(rect.min.x, y), egui::pos2(rect.max.x, y)],
                    egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(0, 255, 0)),
                );
                y += line_spacing / ui.pixels_per_point();
            }
            let line_spacing = 2000.0 * zoom;
            let start_x = rect.min.x - (offset.x * zoom / ui.pixels_per_point()) % line_spacing;
            let mut x = start_x;
            while x < rect.max.x {
                painter.line_segment(
                    [egui::pos2(x, rect.min.y), egui::pos2(x, y)],
                    egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(0, 255, 0)),
                );
                x += line_spacing / ui.pixels_per_point();
            }
            draw_gpu(ui, app, rect);
            // -------------- DEBUG -----------

            // let strokes = app.state.current_file
            //     .as_ref()
            //     .map(|f| f.strokes.clone())
            //     .unwrap_or_default();
            // for stroke in &strokes {
            //     let zoom   = app.state.gpu_view.zoom;
            //     let offset = app.state.gpu_view.top_left;

            //     let screen_min = rect.min + egui::vec2(
            //         (stroke.bbox.min.x - offset.x) * zoom,
            //         (stroke.bbox.min.y - offset.y) * zoom,
            //     );
            //     let screen_max = rect.min + egui::vec2(
            //         (stroke.bbox.max.x - offset.x) * zoom,
            //         (stroke.bbox.max.y - offset.y) * zoom,
            //     );

            //     painter.rect_stroke(
            //         egui::Rect::from_min_max(screen_min, screen_max),
            //         0.0,
            //         egui::Stroke::new(1.0, egui::Color32::RED),
            //         egui::StrokeKind::Outside,
            //     );
            // }
        });
    }
}
