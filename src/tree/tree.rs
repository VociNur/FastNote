// use std::path::PathBuf;

// use eframe::egui;
// use egui_dnd::{dnd, DragDropResponse};

// use crate::projects::fastnote_project::ItemType;

// // #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// // pub struct FlatNode {
// //     pub path: PathBuf,
// //     pub kind: ItemType,
// //     pub depth: usize,
// //     pub is_open: bool,
// // }

// pub fn draw_flat_tree(ui: &mut egui::Ui, flat: &mut Vec<FlatNode>) -> DragDropResponse {
//     dnd(ui, "fastnote_tree").show_vec(flat, |ui, item, handle, _state| {
//         ui.horizontal(|ui| {
//             ui.add_space(item.depth as f32 * 20.0);

//             handle.ui(ui, |ui| {
//                 let name = item.path.file_name().unwrap_or_default().to_string_lossy();
//                 let label = egui::RichText::new(name).size(18.0);
//                 let response = ui.add(egui::Label::new(label).sense(egui::Sense::click()));

//                 // clic gauche
//                 if response.clicked() {
//                     match item.kind {
//                         ItemType::Project | ItemType::Folder => {
//                             println!("Toggle open/close: {:?}", item.path);
//                         }
//                         ItemType::File => {
//                             println!("Ouvrir fichier dans un onglet: {:?}", item.path);
//                         }
//                         ItemType::Page => {
//                             println!("Ouvrir page: {:?}", item.path);
//                         }
//                     }
//                 }

//                 // clic droit
//                 response.context_menu(|ui| match item.kind {
//                     ItemType::Project | ItemType::Folder => {
//                         if ui.button("📁 Nouveau dossier").clicked() {
//                             let new = item.path.join("Nouveau dossier");
//                             std::fs::create_dir_all(&new).ok();
//                             ui.close();
//                         }
//                         if ui.button("📄 Nouveau fichier").clicked() {
//                             let new = item.path.join("Nouveau fichier.fn");
//                             std::fs::write(&new, "{}").ok();
//                             ui.close();
//                         }
//                     }
//                     ItemType::File => {
//                         if ui.button("✏ Renommer").clicked() {
//                             println!("Renommer fichier: {:?}", item.path);
//                             ui.close();
//                         }
//                         if ui.button("🗑 Supprimer").clicked() {
//                             std::fs::remove_file(&item.path).ok();
//                             ui.close();
//                         }
//                     }
//                     ItemType::Page => {
//                         if ui.button("🗑 Supprimer page").clicked() {
//                             println!("Supprimer page: {:?}", item.path);
//                             ui.close();
//                         }
//                     }
//                 });
//             });
//         });
//     })
// }
