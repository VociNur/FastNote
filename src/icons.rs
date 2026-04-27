use egui::epaint::image;



pub struct Icons{

    //File
    pub open_folder: egui::TextureHandle, 
    //Draw
    pub pen: egui::TextureHandle,

    
}
// impl Icons {
//     pub fn load(cc: &egui::Context) -> Self{
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
