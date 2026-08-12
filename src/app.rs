use eframe::egui::{self, Pos2, Rect, ViewportId};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::icons::Icons;
use crate::stylet::stylet_inputs::spawn_pen_thread;
use crate::stylet::stylet_manager::StyletManager;
use crate::ui::ui::draw_gui;
use crate::{
    edition::open_edition_mode, input_manager::InputManager, projects::user_file::UserFile,
    state::State,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct App {
    pub app_have_focus: bool,
    pub state: State,
    pub icons: Icons,
    pub input_manager: InputManager,
    pub stylet_manager: StyletManager,
    pub gpu_rect: Option<Rect>,
    pub x_screen_size: u32,
    pub y_screen_size: u32,
    pub window_state: Arc<Mutex<WindowState>>,
    // pub last_pen_state: Option<PenState>,
    //
    pub ppp: f32,

    pub debug_info: DebugInfo,
    pub nbr_redraw: u32,
}
impl DebugInfo {
    pub fn push(&mut self, msg: impl Into<String>) {
        self.lines.push(msg.into());
    }
}

#[derive(Default)]
pub struct DebugInfo {
    pub lines: Vec<String>,
}

#[derive(Default, Clone)]
pub struct PenState {
    pub pos: egui::Pos2,
    pub pressed: bool,
    pub pressure: f64,
}

#[derive(Default, Clone)]
pub struct WindowState {
    pub pos: egui::Pos2,
    // pub ppp: f32, // pixels_per_point
}
impl App {
    fn default(icons: Icons) -> Self {
        Self {
            app_have_focus: false,
            state: State::default(),
            icons: icons,
            gpu_rect: None,
            window_state: Arc::new(Mutex::new(WindowState::default())),
            stylet_manager: StyletManager::default(),
            input_manager: InputManager::default(),
            x_screen_size: 1,
            y_screen_size: 1,
            // last_pen_state: None,
            // clicks: vec![],
            ppp: 1.,
            debug_info: DebugInfo::default(),
            nbr_redraw: 0,
        }
    }
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, icons: Icons, width: u32, height: u32) -> Self {
        let mut app = App::default(icons);
        let wgpu_state = cc.wgpu_render_state.as_ref().unwrap();
        println!("msaa samples: {:?}", wgpu_state.target_format);
        app.x_screen_size = width;
        app.y_screen_size = height;
        spawn_pen_thread(
            Arc::clone(&app.window_state),
            Arc::clone(&app.stylet_manager.events),
            width,
            height,
            cc.egui_ctx.clone(),
        );
        app
    }

    pub fn user_opened_project(&mut self, path: PathBuf) {
        // self.state.opened_projects.push(UserProject::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unnamed").to_owned(), path.clone(), Color32::BLACK));

        // println!("Opened {:?}", path);
        // let result = self.save_opened_projects();
        // if result.is_err(){
        //     println!("failed to save");
        // }
        println!("Opened");
        println!("path: {:?}", path);
        let res = self
            .state
            .opened_projects
            .load_user_project_from_path(path.clone());
        if res.is_err() {
            println!("Not able to load path ! {:?}", path);
        }
    }

    pub fn user_created_project(&mut self) {
        let dialog = &self.state.new_project_dialog;
        println!("Created");
        println!("name: {}", dialog.name);
        println!("color: {:?}", dialog.color);
        println!("path: {:?}", dialog.path);
        self.state.opened_projects.create_blank_project(
            dialog.path.clone().join(dialog.name.clone()),
            dialog.name.clone(),
            dialog.color,
        );
    }

    pub fn save_state(&mut self) -> anyhow::Result<()> {
        // let path = save_path();
        // let json = serde_json::to_string_pretty(&self.state)?;
        // let tmp_path = path.with_extension("tmp");
        // std::fs::write(&tmp_path, json)?;

        // // 2. Renommer atomiquement → remplace l'ancien fichier d'un coup
        // std::fs::rename(&tmp_path, path)?;

        // Ok(())
        println!("entire save of state deactivated");
        Ok(())
    }

    pub fn open_file(&mut self, file_path: PathBuf) {
        // println!("file path: {:?}", file_path);
        // let json = std::fs::read_to_string(&file_path).unwrap_or_default();
        // let user_file: UserFile = serde_json::from_str(&json).unwrap_or_default()
        let user_file = UserFile::from_path(file_path.clone());
        if user_file.is_err() {
            println!("Could not load file {:?}", file_path);
        }
        self.state.current_file = Some(user_file.unwrap());
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let has_focus = ctx.input(|i| i.focused);
        self.app_have_focus = has_focus;

        self.stylet_manager
            .manage_events(ctx, &mut self.state, &has_focus, &self.gpu_rect);
        // println!("has focus{}", has_focus);
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
            println!("Save state");
        }
        if ctx.input(|i| i.key_pressed(egui::Key::L) && i.modifiers.ctrl) {
            println!("Load state");
        }
        if ctx.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl) {}
        // let window_pos = ctx
        //     .input(|i| i.viewport().outer_rect)
        //     .map(|r| r.min)
        //     .unwrap_or_else(|| {
        //         println!("zero");
        //         egui::Pos2::ZERO
        //     });
        let window2 = frame.winit_window().unwrap().inner_position().unwrap();
        // println!("window_pos2: {:?}", window2);

        {
            let mut w = self.window_state.lock().unwrap();
            w.pos = Pos2::new(window2.x as f32, window2.y as f32);
            // w.ppp = ppp;
        };

        ctx.input(|i| {
            for event in &i.events {
                // println!("Event : {:?}", event);
                self.input_manager
                    .manage_events(&mut self.state, event.clone(), self.ppp);
            }
        });
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.update(ui.ctx(), frame);
        // println!("viewport {}", ViewportId::ROOT);
        self.debug_info.lines = vec![];
        ui.ctx().set_cursor_icon(self.state.cursor_icon);

        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(77, 79, 83);
        ui.ctx().set_visuals(visuals);
        draw_gui(ui, self);
        if self.state.edition_open {
            open_edition_mode(
                &mut self.state.theme,
                ui.ctx(),
                &mut self.state.edition_open,
            );
        }
        self.ppp = ui.pixels_per_point();
        egui::Window::new("Debug Panel")
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-10.0, 10.0))
            .collapsible(false)
            .resizable(true)
            .default_width(250.0)
            .default_height(200.0)
            .show(ui.ctx(), |ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
                ui.visuals_mut().panel_fill = egui::Color32::from_black_alpha(180);

                ui.vertical(|ui| {
                    for line in &self.debug_info.lines {
                        ui.label(line);
                    }
                });
            });
    }

    fn logic(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        _ = (ctx, frame);
    }

    fn on_exit(&mut self) {}

    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(30)
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // NOTE: a bright gray makes the shadows of the windows look weird.
        // We use a bit of transparency so that if the user switches on the
        // `transparent()` option they get immediate results.
        egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).to_normalized_gamma_f32()

        // _visuals.window_fill() would also be a natural choice
    }

    fn persist_egui_memory(&self) -> bool {
        true
    }

    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {}
}
