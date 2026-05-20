use crate::{app::App, ui::{middle::draw_middle, top_bar::draw_top_bar}};


pub fn draw_gui(ui: &mut egui::Ui, app:&mut App){
    draw_top_bar(ui, app);
    draw_middle(ui, app);
   
}
