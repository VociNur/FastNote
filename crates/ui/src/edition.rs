use eframe::egui;
use egui::Context;


use crate::themes::ThemeData;

pub fn open_edition_mode(theme: &mut ThemeData, ctx: &Context, open: &mut bool){
    egui::Window::new("Edition")
        .open(open)
        .resizable(true)
        .show(ctx, |ui|{
            ui.heading("Menu");
            ui.separator();

            // menu_bg
            // menu_fg           

            // menu_selected_bg
            // menu_selected_fg
            ui.horizontal(|ui|{    
                ui.label("Foreground");
                ui.color_edit_button_srgba(&mut theme.menu_fg);
            });
            ui.horizontal(|ui|{
                ui.label("Background");
                ui.color_edit_button_srgba(&mut theme.menu_bg);
            });
            ui.add(egui::Slider::new(&mut theme.menu_size_text, 8.0..=40.0).text("Size menu"));
            ui.add(egui::Slider::new(&mut theme.menu_size_text, 8.0..=40.0).text("Rel size menu"));
            ui.add(egui::Slider::new(&mut theme.space_between_menu_and_ribbon, 0.0..=40.0).text("Space between menu and ribbon"));
            ui.horizontal(|ui|{    
                ui.label("Foreground");
                ui.color_edit_button_srgba(&mut theme.menu_selected_fg);
            });
            ui.horizontal(|ui|{
                ui.label("Selected Background");
                ui.color_edit_button_srgba(&mut theme.menu_selected_bg);
            });

            ui.heading("Ribbon");
            ui.horizontal(|ui|{    
                ui.label("Foreground");
                ui.color_edit_button_srgba(&mut theme.ribbon_fg);
            });
            ui.horizontal(|ui|{    
                ui.label("Background");
                ui.color_edit_button_srgba(&mut theme.ribbon_bg);
            });
            ui.horizontal(|ui|{    
                ui.label("Selected Foreground");
                ui.color_edit_button_srgba(&mut theme.ribbon_selected_fg);
            });
            ui.horizontal(|ui|{    
                ui.label("Selected Background");
                ui.color_edit_button_srgba(&mut theme.ribbon_selected_bg);
            });

            ui.heading("Tree");
            ui.horizontal(|ui|{    
                ui.label("Foreground file");
                ui.color_edit_button_srgba(&mut theme.notebook_tree_text_file_fg);
            });
            ui.horizontal(|ui|{    
                ui.label("Background file");
                ui.color_edit_button_srgba(&mut theme.notebook_tree_text_file_bg);
            });
            ui.horizontal(|ui|{    
                ui.label("Foreground folder");
                ui.color_edit_button_srgba(&mut theme.notebook_tree_text_folder_fg);
            });
            ui.horizontal(|ui|{    
                ui.label("Foreground Folder");
                ui.color_edit_button_srgba(&mut theme.notebook_tree_text_folder_bg);
            });

    });
}
