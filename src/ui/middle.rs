use eframe::egui::{self, Color32};

use egui::{Button, Panel, RichText};

use crate::{
    app::App,
    get_working_path,
    paths::{NOTEBOOK, PROJECT_DEFAULT_FOLDER},
    projects::user_project::UserProject,
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
    let mut opt_to_remove = None;
    ui.vertical(|ui| {
        for project in &mut app.state.opened_projects.projects {
            egui::Frame {
                fill: app
                    .state
                    .theme
                    .ribbon_bg
                    .lerp_to_gamma(*project.get_color(), project.get_color().intensity()),
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
                    // let text_pen =
                    //     RichText::new(project.get_name()).color(app.state.theme.ribbon_fg);
                    ui.vertical(|ui| {
                        ui.horizontal(|ui| {
                            let mut name = project.get_name().to_string();
                            if ui.text_edit_singleline(&mut name).changed() {
                                project.set_name(name);
                            }
                            let mut color = *project.get_color();
                            if ui.color_edit_button_srgba(&mut color).changed() {
                                project.set_color(color);
                            }

                            let icon = egui::Image::new(&app.icons.cross_folder)
                                .fit_to_exact_size(egui::vec2(24.0, 24.0));
                            let cross_button =
                                ui.add_sized([24.0, 24.0], egui::Button::image(icon).frame(false));
                            if cross_button.clicked() {
                                opt_to_remove = Some(project.path.clone());
                            }
                        });
                        let path_str = project
                            .path
                            .canonicalize()
                            .unwrap_or(project.path.clone())
                            .to_string_lossy()
                            .to_string();
                        let path_text = RichText::new(path_str).color(app.state.theme.ribbon_fg);
                        ui.label(path_text);
                    });
                });
            });
        }
    });
    if let Some(path_to_remove) = opt_to_remove {
        println!("Removeing project: {path_to_remove:?}");
        app.state
            .opened_projects
            .unload_user_project_from_path(path_to_remove);
    }
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
    let mut is_showing_tree = false;
    egui::Frame {
        fill: app
            .state
            .theme
            .ribbon_bg
            .lerp_to_gamma(*project.get_color(), project.get_color().intensity()),
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

            //Ici sera un bouton et pas un texte
            // let text_pen = RichText::new(project.get_name()).color(app.state.theme.ribbon_fg);
            // ui.label(text_pen);
            //

            let project_text = RichText::new(project.get_name())
                .size(40f32)
                .color(Color32::BLACK)
                .strong();
            let down_arrow = app.icons.bold_down_arrow.clone();
            let right_arrow = app.icons.bold_right_arrow.clone();
            let header = egui::CollapsingHeader::new(project_text)
                .icon(|ui, openness, response| {
                    let open = openness > 0.5;
                    let notebook_icon = if open { down_arrow } else { right_arrow };

                    // 2. Définir la taille de ton icône (ex: 16x16 pixels)
                    let icon_size = egui::vec2(16.0, 16.0);

                    // 3. Centrer le rectangle de l'icône autour du point central
                    let icon_rect = egui::Rect::from_center_size(response.rect.center(), icon_size);

                    // 4. Dessiner l'image à la place du texte
                    ui.painter().image(
                        notebook_icon.id(), // L'ID de ta texture egui
                        icon_rect,          // Où l'afficher
                        egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV complet (0 à 1)
                        egui::Color32::WHITE, // Teinte (WHITE pour garder les couleurs d'origine)
                    );
                })
                .id_salt(&project.path)
                .default_open(true)
                .show(ui, |_ui| {});
            is_showing_tree = header.fully_open();

            header.header_response.context_menu(|ui| {
                if ui.button("📁 Nouveau dossier").clicked() {
                    create_default_folder(&project.path.join(NOTEBOOK), app);
                    ui.close();
                }
                if ui.button("📄 Nouveau fichier").clicked() {
                    create_default_file(&project.path.join(NOTEBOOK), app);
                    ui.close();
                }
            });
        });
        let project_path = project.path.join(NOTEBOOK);
        if !project_path.exists() {
            println!("Notebook creation...");
            let err = std::fs::create_dir_all(&project_path);
            if err.is_err() {
                println!("could not create notebook ...");
            } else {
                println!("Notebook created !");
            }
        } // Header du notebook — cliquable pour plier/déplier, clic droit pour créer
        if is_showing_tree {
            show_tree(ui, &project_path, app);
        }
    });
}

fn show_tree(ui: &mut egui::Ui, dir: &std::path::Path, app: &mut App) {
    let entries = sorted_entries(dir); // on verra le tri plus tard

    for path in entries {
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let name_text = RichText::new(&name).color(Color32::BLACK).size(24.);
        let is_renaming = app.state.file_tree.renaming.as_ref() == Some(&path);

        if path.is_dir() {
            if is_renaming {
                let response = ui.text_edit_singleline(&mut app.state.file_tree.rename_buf);
                if response.lost_focus()
                    || ui.input(|i| {
                        i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)
                    })
                {
                    let new_path = path.parent().unwrap().join(&app.state.file_tree.rename_buf);
                    std::fs::rename(&path, &new_path).ok();
                    app.state.file_tree.renaming = None;
                }
            } else {
                let header = egui::CollapsingHeader::new(name_text)
                    .id_salt(&path)
                    .show(ui, |ui| {
                        show_tree(ui, &path, app);
                    });
                header.header_response.context_menu(|ui| {
                    if ui.button("📁 Nouveau dossier").clicked() {
                        create_default_folder(&path, app);
                        ui.close();
                    }
                    if ui.button("📄 Nouveau fichier").clicked() {
                        create_default_file(&path, app);
                        ui.close();
                    }
                    if ui.button("✏ Renommer").clicked() {
                        app.state.file_tree.renaming = Some(path.clone());
                        app.state.file_tree.rename_buf = name.clone();
                        ui.close();
                    }
                    if ui.button("🗑 Supprimer").clicked() {
                        std::fs::remove_dir_all(&path).ok();
                        ui.close();
                    }
                });
            }
        } else {
            // Renommage inline
            if is_renaming {
                let response = ui.text_edit_singleline(&mut app.state.file_tree.rename_buf);

                let confirmed = response.lost_focus()
                    || ui.input(|i| i.key_pressed(egui::Key::Enter))
                    || ui.input(|i| i.key_pressed(egui::Key::Escape));

                if confirmed {
                    let new_path = path.parent().unwrap().join(&app.state.file_tree.rename_buf);
                    std::fs::rename(&path, &new_path).ok();
                    app.state.file_tree.renaming = None;
                }
            } else {
                // let text = RichText::new(&name)
                // .size(20.)
                // .color(app.stte.theme.notebook_tree_text_file_fg);
                let button = Button::new(name_text).fill(Color32::from_gray(128));
                let response = ui.add(button);

                if response.clicked() {
                    app.open_file(path.clone());
                }

                // Menu clic droit sur le fichier
                response.context_menu(|ui| {
                    if ui.button("✏ Renommer").clicked() {
                        app.state.file_tree.renaming = Some(path.clone());
                        app.state.file_tree.rename_buf = name.clone();
                        ui.close();
                    }
                    if ui.button("🗑 Supprimer").clicked() {
                        std::fs::remove_file(&path).ok();
                        ui.close();
                    }
                });
            }
        }
    }
}

fn create_default_folder(parent: &std::path::Path, app: &mut App) {
    let name = "Nouveau dossier";
    let path = parent.join(name);
    std::fs::create_dir_all(&path).ok();
    // Active le renommage immédiatement
    app.state.file_tree.renaming = Some(path);
    app.state.file_tree.rename_buf = name.to_string();
}

fn create_default_file(parent: &std::path::Path, app: &mut App) {
    let name = "Nouveau fichier.fn";
    let path = parent.join(name);
    std::fs::write(&path, "{}").ok();
    // Active le renommage immédiatement
    app.state.file_tree.renaming = Some(path);
    app.state.file_tree.rename_buf = name.to_string();
}

fn sorted_entries(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect()
}
