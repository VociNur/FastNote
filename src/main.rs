use FastNote::app::App;
use FastNote::icons::Icons;
use egui::TextureHandle;
use std::io::Cursor;
use std::path::{self, Path};
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
        "Fast Note",
        native_options,
        Box::new(|cc|{
            let icons = Icons{
                pen: load(&cc.egui_ctx, &Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("src/assets/ribbon/pen_icon.png")),
                eraser: load(&cc.egui_ctx, &Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("src/assets/ribbon/eraser_icon.png")),
                open_folder: load(&cc.egui_ctx, &Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("src/assets/menu/file/open_folder.png")),
                notebook: load(&cc.egui_ctx, &Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("src/assets/menu/file/notebook.png")),
            };
            Ok(Box::new(App::new(cc, icons)))
        }),
    )
}
pub fn load(ctx: &egui::Context, path: &Path)->TextureHandle
{
    
    let bytes = std::fs::read(path).expect(&format!("Path not found {:?} /// {:?}", path, path::absolute(path)));//, path::absolute(path)
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
