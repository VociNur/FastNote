use eframe::egui;
use FastNote::app::App;
use FastNote::icons::Icons;
use FastNote::gpu::TriangleRenderer;

fn main() -> eframe::Result {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
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
            let renderer = TriangleRenderer::new(&wgpu_state.device, wgpu_state.target_format);
            wgpu_state
                .renderer
                .write()
                .callback_resources
                .insert(renderer);

            Ok(Box::new(App::new(cc, icons)))
        }),
    )
}
