use eframe::egui;
use egui::{Button, RichText};

use crate::{app::App, pen::{DEFAULT_ERASER, DEFAULT_PEN}, state::MenuMode};
    
pub fn draw_ribbon(ui: &mut egui::Ui, app: &mut App){
    match app.state.get_menu() {
        MenuMode::File => {
            
         }
        MenuMode::Home => {
            
        }
        MenuMode::Draw => {
            draw_draw_ribbon(ui, app);
        }
        _ => {
            
        }
    }
}

pub fn draw_draw_ribbon(ui: &mut egui::Ui, app: &mut App){

    
    let avail = ui.available_width();
    let target_w = avail * 0.98;

    ui.vertical_centered(|ui| {
        ui.set_width(target_w);
        
        ui.set_height(30.);

        egui::Frame {
            fill: app.state.theme.ribbon_bg,
            stroke: egui::Stroke::NONE, // pas de bordure
                corner_radius: egui::CornerRadius { nw: 10, ne: 10, sw: 10, se: 10 },
            ..Default::default()
        }.show(ui, |ui| {
            
            ui.set_width(target_w);
            ui.set_height(30.);
            ui.horizontal_wrapped(|ui| {

        
                let style = ui.style_mut();
                style.visuals.panel_fill = app.state.theme.ribbon_bg;

                // let text_pen = RichText::new("Pen").color(app.state.theme.ribbon_fg);
                // let button_pen = Button::new(text_pen).fill(app.state.theme.ribbon_bg);
                // if ui.add(button_pen).clicked(){
                   // app.state.pen = DEFAULT_PEN;
                // }
                // ui.add(Image::new(app.icons.pen.clone()));

                // ui.add(
                //     egui::Image::from_texture(&app.icons.pen)
                //         .fit_to_exact_size(egui::vec2(24.0, 24.0))
                // );
                //
                for pen in &mut app.state.color_palette.palette{
                    if app.state.color_palette.is_editing{
                        ui.vertical(|ui|{
                            let icon = egui::Image::new(&app.icons.pen)
                                .fit_to_exact_size(egui::vec2(32.0, 32.0));
                            let button_image = ui.add_sized([32.0, 32.0], egui::Button::image(icon).frame(false).fill(pen.color));
                            if button_image.clicked() {
                                app.state.color_palette.pen = pen.clone();
                            }
                            if button_image.hovered(){
                                ui.painter().rect_filled(
                                    button_image.rect,
                                    4.0,
                                    egui::Color32::from_white_alpha(100),
                                );
                            }
                            ui.color_edit_button_srgba(&mut pen.color);
                            
                        });
                    }else{
                        
                        let icon = egui::Image::new(&app.icons.pen)
                            .fit_to_exact_size(egui::vec2(32.0, 32.0));
                        let button_image = ui.add_sized([32.0, 32.0], egui::Button::image(icon).frame(false).fill(pen.color));
                        if button_image.clicked() {
                            app.state.color_palette.pen = pen.clone();
                        }
                        if button_image.hovered(){
                            ui.painter().rect_filled(
                                button_image.rect,
                                4.0,
                                egui::Color32::from_white_alpha(100),
                            );
                        }
                    }
                }
                
                let icon_add = egui::Image::new(&app.icons.plus)
                    .fit_to_exact_size(egui::vec2(32.0, 32.0));
                let button_add = ui.add_sized([32.0, 32.0], egui::Button::image(icon_add).frame(false));
                if button_add.clicked() {
                    app.state.color_palette.add_default_pen_to_palette();
                }
                if button_add.hovered(){
                    ui.painter().rect_filled(
                        button_add.rect,
                        4.0,
                        egui::Color32::from_white_alpha(100),
                    );
                }

                let icon_edit = egui::Image::new(&app.icons.edit_pen)
                    .fit_to_exact_size(egui::vec2(32.0, 32.0));
                let button_edit = ui.add_sized([32.0, 32.0], egui::Button::image(icon_edit).frame(false));
                if button_edit.clicked() {
                    app.state.color_palette.is_editing = !app.state.color_palette.is_editing;
                }
                if button_edit.hovered(){
                    ui.painter().rect_filled(
                        button_edit.rect,
                        4.0,
                        egui::Color32::from_white_alpha(100),
                    );
                }
                //
                // 
                // let text_eraser = RichText::new("Eraser").color(app.state.theme.ribbon_fg);
                // let button_eraser = Button::new(text_eraser).fill(app.state.theme.ribbon_bg);
                // if ui.add(button_eraser).clicked(){
   
                   // app.state.pen = DEFAULT_ERASER;
                // }
            });        
        });
    });
}
