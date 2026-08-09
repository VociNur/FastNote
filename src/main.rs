use FastNote::gpu::main_renderer::MainRenderer;
use eframe::egui;
use FastNote::app::App;
use FastNote::icons::Icons;

fn main() -> eframe::Result {
    env_logger::init();

    let (width, height) = match screen_size::get_primary_screen_size() {
        Ok((width, height)) => {
            println!("Largeur : {}, Hauteur : {}", width, height);
            (width, height)
        }
        Err(e) => {
            println!("Impossible de récupérer la taille de l'écran : {:?}", e);
            (1, 1)
        }
    };
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        multisampling: 4,
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
        Box::new(|cc| {
            let icons = Icons::default_icons(&cc.egui_ctx);

            // Initialise le renderer GPU
            let wgpu_state = cc.wgpu_render_state.as_ref().unwrap();
            let renderer = MainRenderer::new(&wgpu_state.device, wgpu_state.target_format, width as u32, height as u32);
            wgpu_state
                .renderer
                .write()
                .callback_resources
                .insert(renderer);
            Ok(Box::new(App::new(cc, icons, width as u32, height as u32)))
        }),
    )
}
