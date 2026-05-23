use eframe::egui;


use egui::{Button, Panel, RichText};

use crate::{app::App, state::Menu, user_project::UserProject};

pub fn draw_middle(ui: &mut egui::Ui, app: &mut App){
   
    egui::Panel::left("left_panel").show_inside(ui, |ui| {//.frame(egui::Frame{fill: Color32::fromrgb(255, 0, 0), ..Default..default()})
            match app.state.get_menu(){
                Menu::File => draw_file_menu_middle(ui, app),
                Menu::Home => draw_home_menu_middle(ui, app),
                _=>{}
            }
        });

}


//FILE
pub fn draw_file_menu_middle(ui: &mut egui::Ui, app: &mut App){
    ui.horizontal(|ui|{
        let open_folder_icon = egui::Image::new(&app.icons.open_folder)
            .fit_to_exact_size(egui::vec2(32.0, 32.0));
        let open_folder_button = ui.add_sized([32.0, 32.0], egui::Button::image(open_folder_icon).frame(false));
        if open_folder_button.clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_folder() {
                // path is a std::path::PathBuf
                app.open_project(path);
            }
        }
        if open_folder_button.hovered(){
            ui.painter().rect_filled(
                open_folder_button.rect,
                4.0,
                egui::Color32::from_white_alpha(100),
            );
        }

        ui.vertical(|ui|{

            for project in &app.state.opened_projects {
                egui::Frame {
                    
                    fill: app.state.theme.ribbon_bg,
                    stroke: egui::Stroke::NONE, // pas de bordure
                        corner_radius: egui::CornerRadius { nw: 10, ne: 10, sw: 10, se: 10 },
                        inner_margin: egui::Margin { left: 5, right: 5, top: 5, bottom: 5 },
                        outer_margin: egui::Margin { left: 5, right: 5, top: 5, bottom: 5 },
                    ..Default::default()
                }.show(ui, |ui| {
                    ui.horizontal(|ui|{
                        let notebook_icon = egui::Image::new(&app.icons.notebook).fit_to_original_size(0.1);
                        ui.add(notebook_icon);
                        let text_pen = RichText::new(&project.name).color(app.state.theme.ribbon_fg);
                        ui.label(text_pen);
                        
                    });
                });
            }
            
        });
        
    });        
}

//HOME
pub fn draw_home_menu_middle(ui: &mut egui::Ui, app: &mut App){
    let projects = app.state.opened_projects.clone();
    egui::ScrollArea::vertical().show(ui, |ui| {
        projects.iter().for_each(|p| {
            show_notebook(ui , app, &p)
        });
    });
}

pub fn show_notebook(ui: &mut egui::Ui, app: &mut App, project: &UserProject){
                egui::Frame {
                    
                    fill: app.state.theme.ribbon_bg,
                    stroke: egui::Stroke::NONE, // pas de bordure
                        corner_radius: egui::CornerRadius { nw: 10, ne: 10, sw: 10, se: 10 },
                        inner_margin: egui::Margin { left: 5, right: 5, top: 5, bottom: 5 },
                        outer_margin: egui::Margin { left: 5, right: 5, top: 5, bottom: 5 },
                    ..Default::default()
                }.show(ui, |ui| {
                    ui.horizontal(|ui|{
                        let notebook_icon = egui::Image::new(&app.icons.notebook).fit_to_original_size(0.1);
                        ui.add(notebook_icon);
                        let text_pen = RichText::new(&project.name).color(app.state.theme.ribbon_fg);
                        ui.label(text_pen);
                        
                    });
                    show_tree(ui,&project.path, app);
                    // show_tree(ui, app, &project.path);
                });
    
}

fn show_tree(ui: &mut egui::Ui, dir: &std::path::Path, app: &mut App) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().filter_map(|e| e.ok()) {
        // println!("{:?}", entry.file_name());
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        if path.is_dir() {
            // 📁 Dossier → cliquable pour déplier
            let mut text = RichText::new(format!("{name}"));
            text = text.size(20.);
            text = text.color(app.state.theme.notebook_tree_text_folder_fg);
            egui::CollapsingHeader::new(text)

                .show(ui, |ui| {

                    ui.style_mut().text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(28.0, egui::FontFamily::Proportional),
                    );
                    
                    show_tree(ui, &path, app); // récursif → contenu du dossier
                });
        } else {
            // 📄 Fichier → cliquable pour ouvrir
            let mut text = RichText::new(name.clone());
            text = text.size(20.);
            text = text.color(app.state.theme.notebook_tree_text_file_fg);
            let button = Button::new(text).fill(app.state.theme.notebook_tree_text_file_bg);
            let response_button = ui.add(button);
            if response_button.clicked(){
                println!("Name opened: {:?}", name);
                app.open_file(path); 
            }
        }
    }
}
