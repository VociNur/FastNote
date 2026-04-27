use Test_egui::app::App;
use Test_egui::icons::Icons;
use eframe::CreationContext;
use egui::TextureHandle;
use std::io::Cursor;
use std::path::Path;
fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0]), // .with_icon(
                                                  //     // NOTE: Adding an icon is optional
                                                  //     eframe::icon_data::from_png_bytes(
                                                  //         &include_bytes!("../assets/favicon-512x512.png")[..],
                                                  //     )
                                                  //     .expect("Failed to load icon"),
                                                  // )
        ..Default::default()
    };
    eframe::run_native(
        "Egui test",
        native_options,
        Box::new(|cc|{
            let icons = Icons{
                pen: load(&cc.egui_ctx, Path::new("assets/ribbon/pen_icon.png")),
                open_folder: load(&cc.egui_ctx, Path::new("assets/menu/file/open_folder.png")),
            };
            Ok(Box::new(App::new(cc, icons)))
        }),
    )
}
            // let bytes = std::fs::read("assets/ribbon/pen_icon.png")?;
            // let img = image::ImageReader::new(Cursor::new(bytes))
            //     .with_guessed_format()?
            //     .decode()?; // -> DynamicImage

            // let rgba = img.to_rgba8();
            // let size = [rgba.width() as usize, rgba.height() as usize];

            // let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
            // let texture = cc.egui_ctx.load_texture(
            //     "icon",
            //     color_image,
            //     egui::TextureOptions::LINEAR,
            // );
pub fn load(ctx: &egui::Context, path: &Path)->TextureHandle
{
    
    let bytes = std::fs::read(path).unwrap();
    let img = image::ImageReader::new(Cursor::new(bytes))
        .with_guessed_format().unwrap()
        .decode().unwrap(); // -> DynamicImage

    let rgba = img.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];

    let color_image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    let texture = ctx.load_texture(
        "icon",
        color_image,
        egui::TextureOptions::LINEAR,
    );
    texture
}
