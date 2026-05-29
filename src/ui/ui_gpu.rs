use eframe::egui::{self, Pos2, Rect};

use crate::{app::App, gpu::curve_renderer::CurveCallback};



pub fn draw_ui_gpu(ui: &mut egui::Ui, app: &mut App){
    
    if app.state.current_file.is_some() {

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let rect = ui.available_rect_before_wrap();
            app.gpu_rect = Some(rect.clone());
            // println!("Rect : {}", rect);
                // let adj_rect = Rect {min: rect.min, max: Pos2 {x: rect.max.x, y: 1080f32}};
            // println!("Rect : {}", adj_rect);


            // let points = app
            //     .state
            //     .current_file
            //     .as_ref()
            //     .map(|f| f.get_cloned_current_stroke())
            //     .unwrap_or_default();
            ui.painter().rect_filled(rect, 0.0, egui::Color32::WHITE);
            // let current_stroke = app.state.current_file
            // .as_ref()
            // .map(|f| f.get_cloned_current_stroke())
            // .unwrap_or_default();

            // if let Some(last) = current_stroke.last() {
                // println!("{:?}", current_stroke);
                // println!("dernier point: {:?}", last.pos);
            // }
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

            

            let strokes_dirty = app.state.current_file
                .as_ref().unwrap().print_nbr_points();
            let strokes_dirty = app.state.current_file
                .as_ref()
                .map(|f| f.strokes_dirty)
                .unwrap_or(false);

            // Passe les refs, construit les vecs seulement si dirty
            let strokes = if strokes_dirty {
                app.state.current_file
                    .as_ref()
                    .map(|f| f.get_strokes().to_vec()) // clone uniquement quand nécessaire
                    .unwrap_or_default()
            } else {
                vec![] // ignoré dans prepare() si !strokes_dirty
            };

            let current_stroke = app.state.current_file
                .as_ref()
                .map(|f| f.get_current_stroke().to_vec())
                .unwrap_or_default();

            // Après avoir construit le CurveCallback, reset le flag
            if strokes_dirty {
                if let Some(f) = app.state.current_file.as_mut() {
                    f.strokes_dirty = false;
                }
            }

            ui.painter().add(egui_wgpu::Callback::new_paint_callback(
                rect,
                CurveCallback {
                    current_stroke,
                    color: app.state.color_palette.pen.color,
                    strokes,
                    canvas_size: rect.size(),
                    gpu_view: app.state.gpu_view.clone(),
                    strokes_dirty,
                },
            ));

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
