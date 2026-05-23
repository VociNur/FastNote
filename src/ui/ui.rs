use crate::{
    app::App,
    gpu::TriangleCallback,
    ui::{middle::draw_middle, top_bar::draw_top_bar},
};
use eframe::egui::{self, Pos2, Rect};

pub fn draw_gui(ui: &mut egui::Ui, app: &mut App) {
    draw_top_bar(ui, app);
    draw_middle(ui, app);
    // println!("screen_rect: {:?}", ui.ctx().screen_rect());
    // println!("view_poirt: {:?}", ui.ctx().viewport_rect());

    
    egui::CentralPanel::default().show_inside(ui, |ui| {
        let rect = ui.available_rect_before_wrap();
        app.gpu_rect = Some(rect.clone());
        // println!("Rect : {}", rect);
        // let adj_rect = Rect {min: rect.min, max: Pos2 {x: rect.max.x, y: 1080f32}};
        // println!("Rect : {}", adj_rect);
        
        ui.painter().rect_filled(rect, 0.0, egui::Color32::BLACK); 
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            TriangleCallback {
                positions: app.clicks.clone(),
                canvas_size: rect.size(),
            },
        ));
    });
}
