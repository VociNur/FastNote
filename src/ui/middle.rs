use std::path::PathBuf;

use eframe::egui::{self, Color32};

use crate::{
    app::App,
    get_working_path,
    paths::PROJECT_DEFAULT_FOLDER,
    // projects::tree_order::{save_order, sorted_entries},
    projects::fastnote_project::{
        FastnoteFile, FastnoteFolder, FastnoteProject, FolderEntry, ItemType,
    },
    state::MenuMode,
    ui::modal_windows::create_project_modal_window::NewProjectModalWindow,
};
use egui::RichText;

pub fn draw_left(ui: &mut egui::Ui, app: &mut App) {
    egui::Panel::left("left_panel")
        .resizable(true)
        .show(ui, |ui| {
            //.frame(egui::Frame{fill: Color32::fromrgb(255, 0, 0), ..Default..default()})
            match app.state.get_menu() {
                MenuMode::File => draw_file_menu_left(ui, app),
                _ => draw_home_menu_left(ui, app),
            }
        });
    if app.state.get_menu() != MenuMode::File && !app.state.current_fastnote_file.is_none() {
        egui::Panel::left("left_panel_secondary")
            .resizable(true)
            .show(ui, |ui| {
                draw_page_menu_left(ui, app);
            });
    }
    // Second panneau à gauche (comme OneNote)
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
                    .lerp_to_gamma(project.get_color(), project.get_color().intensity()),
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
                                project.set_name(name).unwrap();
                            }
                            let mut color = project.get_color();
                            if ui.color_edit_button_srgba(&mut color).changed() {
                                project.set_color(color).unwrap();
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
            .unload_fastnote_project_from_path(path_to_remove);
    }
}
pub fn draw_file_menu_left(ui: &mut egui::Ui, app: &mut App) {
    ui.vertical(|ui| {
        ui.add_space(10f32);
        let new_folder_icon =
            egui::Image::new(&app.icons.plus).fit_to_exact_size(egui::vec2(32.0, 32.0));
        let new_folder_button = ui.add_sized(
            [32.0, 32.0],
            egui::Button::image(new_folder_icon).frame(false),
        );
        if new_folder_button.clicked() {
            let path = get_working_path().join(PROJECT_DEFAULT_FOLDER);
            std::fs::create_dir_all(&path).ok();

            if let Some(path) = rfd::FileDialog::new().set_directory(path).pick_folder() {
                let dialog = NewProjectModalWindow {
                    path,
                    name: "".to_owned(),
                    color: Color32::BLACK,
                };
                app.state.modal_window =
                    super::modal_windows::modal_window::ModalWindow::NewProject(dialog);
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
    // let projects_manager = app.state.opened_projects.clone();
    // egui::ScrollArea::vertical().show(ui, |ui| {
    //     projects_manager
    //         .projects
    //         .iter()
    //         .for_each(|p| show_notebook(ui, app, p));
    // });
    show_notebooks(ui, app);
}
pub fn show_notebooks(ui: &mut egui::Ui, app: &mut App) {
    let mut flat = Vec::new();
    for project in app.state.opened_projects.clone().projects.iter() {
        flatten_project(project, &mut flat);
    }
    // println!("flatten {}", flat.len());
    draw_tree(ui, &mut flat, app);
}
// pub fn show_notebook(ui: &mut egui::Ui, app: &mut App, project: &UserProject) {
//     let mut is_showing_tree = false;
//     egui::Frame {
//         fill: app
//             .state
//             .theme
//             .ribbon_bg
//             .lerp_to_gamma(*project.get_color(), project.get_color().intensity()),
//         stroke: egui::Stroke::NONE, // pas de bordure
//         corner_radius: egui::CornerRadius {
//             nw: 10,
//             ne: 10,
//             sw: 10,
//             se: 10,
//         },
//         inner_margin: egui::Margin {
//             left: 5,
//             right: 5,
//             top: 5,
//             bottom: 5,
//         },
//         outer_margin: egui::Margin {
//             left: 5,
//             right: 5,
//             top: 5,
//             bottom: 5,
//         },
//         ..Default::default()
//     }
//     .show(ui, |ui| {
//         ui.horizontal(|ui| {
//             let notebook_icon = egui::Image::new(&app.icons.notebook).fit_to_original_size(0.1);
//             ui.add(notebook_icon);

//             //Ici sera un bouton et pas un texte
//             // let text_pen = RichText::new(project.get_name()).color(app.state.theme.ribbon_fg);
//             // ui.label(text_pen);
//             //

//             let project_text = RichText::new(project.get_name())
//                 .size(40f32)
//                 .color(Color32::BLACK)
//                 .strong();
//             let down_arrow = app.icons.bold_down_arrow.clone();
//             let right_arrow = app.icons.bold_right_arrow.clone();
//             let header = egui::CollapsingHeader::new(project_text)
//                 .icon(|ui, openness, response| {
//                     let open = openness > 0.5;
//                     let notebook_icon = if open { down_arrow } else { right_arrow };

//                     // 2. Définir la taille de ton icône (ex: 16x16 pixels)
//                     let icon_size = egui::vec2(16.0, 16.0);

//                     // 3. Centrer le rectangle de l'icône autour du point central
//                     let icon_rect = egui::Rect::from_center_size(response.rect.center(), icon_size);

//                     // 4. Dessiner l'image à la place du texte
//                     ui.painter().image(
//                         notebook_icon.id(), // L'ID de ta texture egui
//                         icon_rect,          // Où l'afficher
//                         egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), // UV complet (0 à 1)
//                         egui::Color32::WHITE, // Teinte (WHITE pour garder les couleurs d'origine)
//                     );
//                 })
//                 .id_salt(&project.path)
//                 .default_open(true)
//                 .show(ui, |_ui| {});
//             is_showing_tree = header.fully_open();

//             header.header_response.context_menu(|ui| {
//                 if ui.button("📁 Nouveau dossier").clicked() {
//                     create_default_folder(&project.path.join(NOTEBOOK), app);
//                     ui.close();
//                 }
//                 if ui.button("📄 Nouveau fichier").clicked() {
//                     create_default_file(&project.path.join(NOTEBOOK), app);
//                     ui.close();
//                 }
//             });
//         });
//         let project_path = project.path.join(NOTEBOOK);
//         if !project_path.exists() {
//             println!("Notebook creation...");
//             let err = std::fs::create_dir_all(&project_path);
//             if err.is_err() {
//                 println!("could not create notebook ...");
//             } else {
//                 println!("Notebook created !");
//             }
//         } // Header du notebook — cliquable pour plier/déplier, clic droit pour créer
//         if is_showing_tree {
//             show_tree(ui, &project_path, app);
//         }
//     });
// }

// fn show_tree(ui: &mut egui::Ui, dir: &std::path::Path, app: &mut App) {
//     let mut items = sorted_entries(dir);
//     let dir = dir.to_path_buf();

//     let response = egui_dnd::dnd(ui, dir.to_str().unwrap_or("tree")).show_vec(
//         &mut items,
//         |ui, item, handle, state| {
//             let path = &item.path;
//             let name = path
//                 .file_stem()
//                 .unwrap_or_default()
//                 .to_string_lossy()
//                 .to_string();
//             // let is_renaming = app.state.file_tree.renaming.as_ref() == Some(path);
//             if !item.is_dir {
//                 if is_renaming {
//                     // Renommage : pas de drag, juste le champ texte
//                     let response = ui.text_edit_singleline(&mut app.state.file_tree.rename_buf);
//                     if response.lost_focus()
//                         || ui.input(|i| {
//                             i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)
//                         })
//                     {
//                         let new_path = path.parent().unwrap().join(&app.state.file_tree.rename_buf);
//                         std::fs::rename(path, &new_path).ok();
//                         app.state.file_tree.renaming = None;
//                     }
//                 } else {
//                     // let text = RichText::new(&name)
//                     // .size(20.)
//                     // .color(app.stte.theme.notebook_tree_text_file_fg);
//                     //

//                     let label = RichText::new(format!("{}", name))
//                         .color(Color32::BLACK)
//                         .size(24.);
//                     let button = Button::new(label).fill(Color32::from_gray(128));
//                     let response = ui.add(button);

//                     if response.clicked() {
//                         app.open_file(path.clone());
//                     }

//                     // Menu clic droit sur le fichier
//                     response.context_menu(|ui| {
//                         if ui.button("✏ Renommer").clicked() {
//                             app.state.file_tree.renaming = Some(path.clone());
//                             app.state.file_tree.rename_buf = name.clone();
//                             ui.close();
//                         }
//                         if ui.button("🗑 Supprimer").clicked() {
//                             std::fs::remove_file(&path).ok();
//                             ui.close();
//                         }
//                     });
//                 }
//             } else {
//                 if is_renaming {
//                     // Renommage : pas de drag, juste le champ texte
//                     let response = ui.text_edit_singleline(&mut app.state.file_tree.rename_buf);
//                     if response.lost_focus()
//                         || ui.input(|i| {
//                             i.key_pressed(egui::Key::Enter) || i.key_pressed(egui::Key::Escape)
//                         })
//                     {
//                         let new_path = path.parent().unwrap().join(&app.state.file_tree.rename_buf);
//                         std::fs::rename(path, &new_path).ok();
//                         app.state.file_tree.renaming = None;
//                     }
//                 } else if item.is_dir {
//                     let canonical = path.canonicalize().unwrap_or(path.clone());
//                     let is_open = app.state.file_tree.open_dirs.contains(&canonical);

//                     ui.horizontal(|ui| {
//                         // Seule cette ligne est la poignée — pas les enfants
//                         handle.ui(ui, |ui| {
//                             let arrow = if is_open { "▼" } else { "▶" };
//                             let label = RichText::new(format!("{} {}", arrow, name))
//                                 .color(Color32::BLACK)
//                                 .size(24.);

//                             // let response = ui.label(label);
//                             let response = ui.add(
//                                 egui::Label::new(
//                                     RichText::new(format!("{} {}", arrow, name))
//                                         .color(Color32::BLACK)
//                                         .size(24.),
//                                 )
//                                 .sense(egui::Sense::click()),
//                             );
//                             if response.clicked() {
//                                 println!("clicked");
//                                 if is_open {
//                                     println!("opened");
//                                     app.state.file_tree.open_dirs.remove(path);
//                                 } else {
//                                     let res_path = path.canonicalize();
//                                     if res_path.is_err() {
//                                         println!("Could not canonicalize");
//                                     } else {
//                                         let can_path = res_path.unwrap();
//                                         app.state.file_tree.open_dirs.insert(can_path);
//                                     }
//                                     println!("try to open");
//                                 }
//                             }

//                             response.context_menu(|ui| {
//                                 if ui.button("📁 Nouveau dossier").clicked() {
//                                     create_default_folder(path, app);
//                                     ui.close();
//                                 }
//                                 if ui.button("📄 Nouveau fichier").clicked() {
//                                     create_default_file(path, app);
//                                     ui.close();
//                                 }
//                                 if ui.button("✏ Renommer").clicked() {
//                                     app.state.file_tree.renaming = Some(path.clone());
//                                     app.state.file_tree.rename_buf = name.clone();
//                                     ui.close();
//                                 }
//                                 if ui.button("🗑 Supprimer").clicked() {
//                                     std::fs::remove_dir_all(path).ok();
//                                     ui.close();
//                                 }
//                             });
//                         });
//                     });

//                     // Les enfants sont EN DEHORS du handle — ils ont leur propre dnd
//                     if is_open {
//                         ui.indent(path, |ui| {
//                             show_tree(ui, path, app);
//                         });
//                     }
//                 }
//             }
//         },
//     );

//     if response.is_drag_finished() {
//         save_order(&dir, &items);
//     }

//     if let Some(dropped) = response.final_update() {
//         // TODO : drop cross-dossier
//     }
// }

// fn create_default_folder(parent: &std::path::Path, app: &mut App) {
//     let name = "Nouveau dossier";
//     let path = parent.join(name);
//     std::fs::create_dir_all(&path).ok();
//     // Active le renommage immédiatement
//     app.state.file_tree.renaming = Some(path);
//     app.state.file_tree.rename_buf = name.to_string();
// }

// fn create_default_file(parent: &std::path::Path, app: &mut App) {
//     let name = "Nouveau fichier.fn";
//     let path = parent.join(name);
//     std::fs::write(&path, "{}").ok();
//     // Active le renommage immédiatement
//     app.state.file_tree.renaming = Some(path);
//     app.state.file_tree.rename_buf = name.to_string();
// }
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FlatNode {
    pub path: PathBuf,
    pub name: String,
    pub kind: ItemType,
    pub depth: usize,
    pub is_open: bool,
}
pub fn flatten_project(project: &FastnoteProject, out: &mut Vec<FlatNode>) {
    out.push(FlatNode {
        path: project.path.clone(),
        name: project.manifest.name.clone(),
        kind: ItemType::Project,
        depth: 0,
        is_open: project.manifest.is_open,
    });
    if project.manifest.is_open {
        flatten_folder_entries(&project.children, 1, out);
    }
}

fn flatten_folder_entries(entries: &Vec<FolderEntry>, depth: usize, out: &mut Vec<FlatNode>) {
    for entry in entries {
        match entry {
            FolderEntry::Folder(f) => {
                out.push(FlatNode {
                    path: f.path.clone(),
                    name: f.manifest.name.clone(),
                    kind: ItemType::Folder,
                    depth,
                    is_open: f.manifest.is_open,
                });
                if f.manifest.is_open {
                    flatten_folder_entries(&f.children, depth + 1, out);
                }
            }

            FolderEntry::File(f) => {
                out.push(FlatNode {
                    path: f.path.clone(),
                    name: f.manifest.name.clone(),
                    kind: ItemType::File,
                    depth,
                    is_open: false,
                });
            }
        }
    }
}

pub fn toggle_item_open(app: &mut App, item: &FlatNode) {
    match item.kind {
        ItemType::Project => {
            if let Some(project) = find_project_mut(app, item.path.clone()) {
                project.manifest.is_open = !project.manifest.is_open;
                let response_save = project.save();
                if let Err(err) = response_save {
                    app.push_unsafe_minute_error(err.to_string());
                }
            }
        }
        ItemType::Folder => {
            if let Some(folder) = find_folder_mut(app, item.path.clone()) {
                folder.manifest.is_open = !folder.manifest.is_open;
                let response_save = folder.save();
                if let Err(err) = response_save {
                    app.push_unsafe_minute_error(err.to_string());
                }
            }
        }
        ItemType::File => app.state.current_fastnote_file = Some(item.path.clone()),
        _ => {}
    }
}
pub fn find_project_mut(app: &mut App, path: PathBuf) -> Option<&mut FastnoteProject> {
    app.state
        .opened_projects
        .projects
        .iter_mut()
        .find(|p| p.path == path)
}
fn find_folder_mut(app: &mut App, path: PathBuf) -> Option<&mut FastnoteFolder> {
    for project in &mut app.state.opened_projects.projects {
        if let Some(folder) = find_folder_rec(&mut project.children, path.clone()) {
            return Some(folder);
        }
    }
    None
}

fn find_folder_rec(entries: &mut Vec<FolderEntry>, path: PathBuf) -> Option<&mut FastnoteFolder> {
    for entry in entries {
        match entry {
            FolderEntry::Folder(f) => {
                if f.path == path {
                    return Some(f);
                }
                if let Some(found) = find_folder_rec(&mut f.children, path.clone()) {
                    return Some(found);
                }
            }
            _ => {}
        }
    }
    None
}
fn find_file_mut(app: &mut App, path: PathBuf) -> Option<&mut FastnoteFile> {
    for project in &mut app.state.opened_projects.projects {
        if let Some(file) = find_file_rec(&mut project.children, path.clone()) {
            return Some(file);
        }
    }
    None
}
fn find_file_rec(entries: &mut Vec<FolderEntry>, path: PathBuf) -> Option<&mut FastnoteFile> {
    for entry in entries {
        match entry {
            FolderEntry::File(f) => {
                if f.path == path {
                    return Some(f);
                }
            }
            FolderEntry::Folder(folder) => {
                if let Some(found) = find_file_rec(&mut folder.children, path.clone()) {
                    return Some(found);
                }
            }
        }
    }
    None
}

pub fn draw_tree(
    ui: &mut egui::Ui,
    flat: &mut Vec<FlatNode>,
    app: &mut App,
) -> egui_dnd::DragDropResponse {
    egui_dnd::dnd(ui, "fastnote_tree").show_vec(flat, |ui, item, handle, _state| {
        ui.horizontal(|ui| {
            ui.add_space(item.depth as f32 * 20.0);

            if item.kind == ItemType::Project || item.kind == ItemType::Folder {
                let open = if item.kind == ItemType::Project {
                    let project = find_project_mut(app, item.path.clone());
                    if project.is_none() {
                        app.push_minute_error(
                            ui,
                            format!("project is not find: {}", item.path.to_string_lossy()),
                        );
                        false
                    } else {
                        project.unwrap().manifest.is_open
                    }
                } else {
                    let folder = find_folder_mut(app, item.path.clone());
                    if folder.is_none() {
                        app.push_minute_error(
                            ui,
                            format!("folder is not find: {}", item.path.to_string_lossy()),
                        );
                        false
                    } else {
                        folder.unwrap().manifest.is_open
                    }
                };
                let arrow_icon = if open {
                    &app.icons.bold_down_arrow
                } else {
                    &app.icons.bold_right_arrow
                };
                let arrow = egui::Image::new(arrow_icon).fit_to_exact_size(egui::vec2(16.0, 16.0));

                let arrow_resp =
                    ui.add_sized([16.0, 16.0], egui::Button::image(arrow).frame(false));

                if arrow_resp.clicked() {
                    toggle_item_open(app, item);
                }
            } else {
                ui.add_space(16.);
            }
            handle.ui(ui, |ui| {
                let label = egui::RichText::new(&item.name)
                    .size(18.0)
                    .color(egui::Color32::BLACK);

                let response = ui.add(egui::Label::new(label).sense(egui::Sense::click()));
                if response.clicked() {
                    if item.kind == ItemType::File {
                        toggle_item_open(app, item);
                    }
                }
                response.context_menu(|ui| {
                    if ui.button("📁 Nouveau dossier").clicked() {
                        create_fastnote_folder(item.path.clone(), app);
                        ui.close();
                    }
                    if ui.button("📄 Nouveau fichier").clicked() {
                        create_fastnote_file(item.path.clone(), app);
                        ui.close();
                    }
                    if item.kind != ItemType::Project {
                        if ui.button("✏ Renommer").clicked() {
                            rename_fastnote(item.path.clone(), app);
                            ui.close();
                        }
                    }
                    if ui.button("🗑 Supprimer").clicked() {
                        delete_fastnote(item.path.clone(), app);
                        ui.close();
                    }
                    // if arrow.clicked() {
                    // toggle_item_open(app, item);
                });
            });
        });
    })
}
pub fn create_fastnote_folder(path: PathBuf, app: &mut App) {
    app.state.modal_window = super::modal_windows::modal_window::ModalWindow::NewFolder(
        super::modal_windows::new_folder_modal_window::NewFolderModalWindow::new(path),
    )
}
pub fn create_fastnote_file(path: PathBuf, app: &mut App) {
    app.state.modal_window = super::modal_windows::modal_window::ModalWindow::NewFile(
        super::modal_windows::new_file_modal_window::NewFileModalWindow::new(path),
    )
}
pub fn rename_fastnote(path: PathBuf, app: &mut App) {
    app.state.modal_window = super::modal_windows::modal_window::ModalWindow::Rename(
        super::modal_windows::rename_modal_window::RenameModalWindow::new(path),
    )
}
pub fn delete_fastnote(path: PathBuf, app: &mut App) {
    app.state.modal_window = super::modal_windows::modal_window::ModalWindow::NewFolder(
        super::modal_windows::new_folder_modal_window::NewFolderModalWindow::new(path),
    )
}
pub fn draw_page_menu_left(ui: &mut egui::Ui, app: &mut App) {
    // On récupère juste le path du fichier
    let Some(path_file) = app.state.current_fastnote_file.clone() else {
        return;
    };

    // On récupère le fichier (emprunt mutable court)
    let Some(file) = find_file_mut(app, path_file.clone()) else {
        app.push_minute_error(ui, "Could not load file for left page menu.");
        return;
    };

    // --- 1) On clone les pages AVANT la closure ---
    let mut pages: Vec<(PathBuf, String)> = file
        .children
        .iter()
        .map(|entry| {
            (
                entry.path.clone(),
                entry
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            )
        })
        .collect();
    ui.vertical(|ui| {
        ui.heading("Pages");
        ui.add_space(8.0);

        // Bouton + Page (juste un print)
        if ui.button("+ Page").clicked() {
            println!("(DEBUG) Create page");
            app.state.modal_window = crate::ui::modal_windows::modal_window::ModalWindow::NewPage(
                crate::ui::modal_windows::new_page_modal_window::NewPageModalWindow {
                    parent: app.state.current_fastnote_file.clone().unwrap(),
                    folder_name: "".to_owned(),
                    display_name: "".to_owned(),
                },
            )
        }

        ui.separator();

        // --- 2) DnD sur les pages clonées ---
        let response = egui_dnd::dnd(ui, "pages_dnd").show_vec(
            &mut pages,
            |ui, (path, name), handle, _state| {
                ui.horizontal(|ui| {
                    handle.ui(ui, |ui| {
                        let label = egui::RichText::new(name.clone())
                            .size(18.0)
                            .color(Color32::BLACK);

                        let response = ui.add(egui::Label::new(label).sense(egui::Sense::click()));

                        // Clic gauche → print
                        if response.clicked() {
                            println!("(DEBUG) Opening page: {}", name);
                            app.state.current_fastnote_page = Some(path.clone());
                        }

                        // Clic droit → print
                        response.context_menu(|ui| {
                            if ui.button("✏ Renommer").clicked() {
                                println!("(DEBUG) Rename page: {}", name);
                                ui.close();
                            }

                            if ui.button("🗑 Supprimer").clicked() {
                                println!("(DEBUG) Delete page: {}", name);
                                ui.close();
                            }
                        });
                    });
                });
            },
        );

        // On peut print le résultat du DnD
        if response.is_drag_finished() {
            println!("(DEBUG) Pages reordered:");
            for (i, (_, name)) in pages.iter().enumerate() {
                println!("  {} → {}", i, name);
            }
        }
    });
}
