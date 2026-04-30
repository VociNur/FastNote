use egui::{Button, RichText};

use crate::{app::App, state::{Menu, State}};



pub fn draw_menu(ui: &mut egui::Ui, app:&mut App){
         
        egui::MenuBar::new().ui(ui, |ui| {
            
            for menu_mode in [Menu::File, Menu::Home, Menu::Insert, Menu::Draw, Menu::History, Menu::View, Menu::Edition]{
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
