use eframe::egui;
use image::ImageReader;
use std::{
    io::Cursor,
    path::{self, Path},
};

use egui::TextureHandle;

pub struct Icons {
    //File
    pub open_folder: egui::TextureHandle,
    pub plus: egui::TextureHandle,
    pub cross_folder: egui::TextureHandle,
    //Draw
    pub pen: egui::TextureHandle,
    pub eraser: egui::TextureHandle,
    pub notebook: egui::TextureHandle,
    //Left
    
    pub bold_right_arrow: egui::TextureHandle,
    pub down_arrow: egui::TextureHandle,
    pub bold_down_arrow: egui::TextureHandle,

    pub edit_pen: egui::TextureHandle,
}

impl Icons {
    pub fn default_icons(ctx: &egui::Context) -> Self {
        Self {
            pen: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/ribbon/pen_icon.png"),
            ),
            eraser: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/ribbon/eraser_icon.png"),
            ),
            open_folder: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/menu/file/open_folder.png"),
            ),
            plus: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/menu/file/plus.png"),
                
            ),
            cross_folder: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/menu/file/cross.png"),
                
            ),
            notebook: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/menu/file/notebook.png"),
            ),
            bold_right_arrow: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/left/bold_right_arrow.png"),
            ),
            down_arrow: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/left/down_arrow.png"),
            ),
            bold_down_arrow: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/left/bold_down_arrow.png"),
            ),
            edit_pen: load(
                ctx,
                &Path::new(env!("CARGO_MANIFEST_DIR")).join("src/assets/ribbon/edit_pen.png"),
            ),
        }
    }
}

// impl Icons {
//     pub fn loadi::Context) -> Self{
//          // Self{
//          //     pen: load_icon(ctx, include_bytes!("assets/ribbon/pen_icon.png")),
//          // }
//         // Charge une image depuis le disque AVANT le lancement
//         let bytes = std::fs::read("assets/ribbon/pen_icon.png").unwrap();
//         let image = image::load_from_memory(&bytes).unwrap().to_rgba8();
//         let size = [image.width() as usize, image.height() as usize];

//         let color_image = egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw());
//         let texture = cc.egui_ctx.load_texture(
//             "icon",
//             color_image,
//             egui::TextureOptions::LINEAR,
//         );
//         Self{
//             pen: texture,
//         }
//     }
// }

// fn load_icon(ctx: &egui::Context, bytes:&[u8]) -> egui::TextureHandle {
//     let image = image::load_from_memory(bytes).unwrap().to_rgba8();

// }
pub fn load(ctx: &egui::Context, path: &Path) -> TextureHandle {
    let bytes = std::fs::read(path).expect(&format!(
        "Path not found {:?} /// {:?}",
        path,
        path::absolute(path)
    )); //, path::absolute(path)
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap(); // -> DynamicImage

    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    let texture = ctx.load_texture("icon", color_image, egui::TextureOptions::LINEAR);
    texture
}
