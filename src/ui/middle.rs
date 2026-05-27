use eframe::egui;

use egui::{Button, Panel, RichText};

use crate::{
    app::App, get_working_path, paths::{NOTEBOOK, PROJECT_DEFAULT_FOLDER}, projects::user_project::UserProject,
    state::MenuMode,
};

pub fn draw_left(ui: &mut egui::Ui, app: &mut App) {
    egui::Panel::left("left_panel").show_inside(ui, |ui| {
        //.frame(egui::Frame{fill: Color32::fromrgb(255, 0, 0), ..Default..default()})
        match app.state.get_menu() {
            MenuMode::File => draw_file_menu_left(ui, app),
            MenuMode::Home => draw_home_menu_left(ui, app),
            _ => {}
        }
    });
}

//FILE
pub fn draw_file_menu_middle(ui: &mut egui::Ui, app: &mut App) {
    ui.vertical(|ui| {
        for project in &app.state.opened_projects.projects {
            egui::Frame {
                fill: app
                    .state
                    .theme
                    .ribbon_bg
                    .lerp_to_gamma(project.color, project.color.intensity()),
                stroke: egui::Stroke::NONE, // pas de bordure
                corner_radius: egui::CornerRadius {
                    nw: 10,
                    ne: 10,
                    sw: 10,
                    se: 10,
                },
                inner_margin: egui::Margin {
                    left: 5,
                    right: 5,
                    top: 5,
                    bottom: 5,
                },
                outer_margin: egui::Margin {
                    left: 5,
                    right: 5,
                    top: 5,
                    bottom: 5,
                },
                ..Default::default()
            }
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let notebook_icon =
                        egui::Image::new(&app.icons.notebook).fit_to_original_size(0.1);
                    ui.add(notebook_icon);
                    let text_pen = RichText::new(&project.name).color(app.state.theme.ribbon_fg);
                    ui.label(text_pen);
                });
            });
        }
    });
}
pub fn draw_file_menu_left(ui: &mut egui::Ui, app: &mut App) {
    ui.vertical(|ui| {
        ui.add_space(10f32);
        let new_folder_icon =
            egui::Image::new(&app.icons.new_folder).fit_to_exact_size(egui::vec2(32.0, 32.0));
        let new_folder_button = ui.add_sized(
            [32.0, 32.0],
            egui::Button::image(new_folder_icon).frame(false),
        );
        if new_folder_button.clicked() {
            let path = get_working_path().join(PROJECT_DEFAULT_FOLDER);
            std::fs::create_dir_all(&path).ok();

            if let Some(path) = rfd::FileDialog::new().set_directory(path).pick_folder() {
                app.state.new_project_dialog.path = path;

                app.state.new_project_dialog.open = true;
            }
        }
        ui.add_space(10f32);
        let open_folder_icon =
            egui::Image::new(&app.icons.open_folder).fit_to_exact_size(egui::vec2(32.0, 32.0));
        let open_folder_button = ui.add_sized(
            [32.0, 32.0],
            egui::Button::image(open_folder_icon).frame(false),
        );
        if open_folder_button.clicked() {
            let path = get_working_path().join(PROJECT_DEFAULT_FOLDER);
            std::fs::create_dir_all(&path).ok();

            if let Some(path) = rfd::FileDialog::new().set_directory(path).pick_folder() {
                app.user_opened_project(path);
            }
        }

        ui.add_space(10f32);
        let manage_icon =
            egui::Image::new(&app.icons.open_folder).fit_to_exact_size(egui::vec2(32.0, 32.0));
        let manage_button =
            ui.add_sized([32.0, 32.0], egui::Button::image(manage_icon).frame(false));
        if manage_button.clicked() {
            let path = get_working_path().join(PROJECT_DEFAULT_FOLDER);
            std::fs::create_dir_all(&path).ok();

            let res = std::process::Command::new("xdg-open").arg(&path).spawn();
            if res.is_err() {
                println!("Could not lauch xdg-open")
            }
        }
        if open_folder_button.hovered() {
            ui.painter().rect_filled(
                open_folder_button.rect,
                4.0,
                egui::Color32::from_white_alpha(100),
            );
        }
        if new_folder_button.hovered() {
            ui.painter().rect_filled(
                new_folder_button.rect,
                4.0,
                egui::Color32::from_white_alpha(100),
            );
        }
        if manage_button.hovered() {
            ui.painter().rect_filled(
                manage_button.rect,
                4.0,
                egui::Color32::from_white_alpha(100),
            );
        }
    });
}

//HOME
pub fn draw_home_menu_left(ui: &mut egui::Ui, app: &mut App) {
    let projects_manager = app.state.opened_projects.clone();
    egui::ScrollArea::vertical().show(ui, |ui| {
        projects_manager
            .projects
            .iter()
            .for_each(|p| show_notebook(ui, app, &p));
    });
}

pub fn show_notebook(ui: &mut egui::Ui, app: &mut App, project: &UserProject) {
    egui::Frame {
        fill: app.state.theme.ribbon_bg,
        stroke: egui::Stroke::NONE, // pas de bordure
        corner_radius: egui::CornerRadius {
            nw: 10,
            ne: 10,
            sw: 10,
            se: 10,
        },
        inner_margin: egui::Margin {
            left: 5,
            right: 5,
            top: 5,
            bottom: 5,
        },
        outer_margin: egui::Margin {
            left: 5,
            right: 5,
            top: 5,
            bottom: 5,
        },
        ..Default::default()
    }
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            let notebook_icon = egui::Image::new(&app.icons.notebook).fit_to_original_size(0.1);
            ui.add(notebook_icon);
            let text_pen = RichText::new(&project.name).color(app.state.theme.ribbon_fg);
            ui.label(text_pen);
        });
        show_tree(ui, &project.path.join(NOTEBOOK), app);
        // show_tree(ui, app, &project.path);
    });
}

fn show_tree(ui: &mut egui::Ui, dir: &std::path::Path, app: &mut App) {
    for entry in std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
    {
        // println!("{:?}", entry.file_name());
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy();

        if path.is_dir() {
            // 📁 Dossier → cliquable pour déplier
            let mut text = RichText::new(format!("{name}"));
            text = text.size(20.);
            text = text.color(app.state.theme.notebook_tree_text_folder_fg);
            egui::CollapsingHeader::new(text).show(ui, |ui| {
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
            if response_button.clicked() {
                println!("Name opened: {:?}", name);
                app.open_file(path);
            }
        }
    }
}
