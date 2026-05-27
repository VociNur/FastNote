use eframe::egui;
use egui::{Button, RichText};

use crate::{app::App, state::MenuMode};



pub fn draw_menu(ui: &mut egui::Ui, app:&mut App){
         
        egui::MenuBar::new().ui(ui, |ui| {
            
            for menu_mode in [MenuMode::File, MenuMode::Home, MenuMode::Insert, MenuMode::Draw, MenuMode::History, MenuMode::View, MenuMode::Edition]{
                let mut bg_color = app.state.theme.menu_bg;
                let mut fg_color = app.state.theme.menu_fg;
                if menu_mode == app.state.get_menu(){
                    bg_color = app.state.theme.menu_selected_bg;
                    fg_color = app.state.theme.menu_selected_fg;
                }
            
                let text = RichText::new(menu_mode.as_str()).size(app.state.theme.menu_size_text).color(fg_color);
                let button = Button::new(text).fill(bg_color);

                if ui.add(button).clicked(){//.fill(color)
                    app.state.set_menu(menu_mode, ui.ctx());
                }
            
            }
            // egui::widgets::global_theme_preference_buttons(ui);
        });

    
    
}
