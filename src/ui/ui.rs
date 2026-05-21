use crate::{
    app::App,
    gpu::TriangleCallback,
    ui::{middle::draw_middle, top_bar::draw_top_bar},
};
use eframe::egui;

pub fn draw_gui(ui: &mut egui::Ui, app: &mut App) {
    draw_top_bar(ui, app);
    draw_middle(ui, app);

    
    let rect = ui.available_rect_before_wrap();
    ui.painter().add(egui_wgpu::Callback::new_paint_callback(
        rect,
        TriangleCallback,
    ));
}
