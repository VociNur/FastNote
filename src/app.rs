use chrono::Local;
use eframe::egui::{self, Pos2};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    create_backup_notes_directory, edition::open_edition_mode, errors::DisplayError, get_last_save,
    projects::loaded_page::LoadedPage, rect_points_to_pixels, state::State, ui::ui::draw_gui,
};
use crate::{event_managers::finger_manager::FingerManager, icons::Icons};
use crate::{
    stylet::stylet_inputs::spawn_pen_thread,
    ui::modal_windows::create_project_modal_window::NewProjectModalWindow,
};
use crate::{stylet::stylet_manager::StyletManager, ui::ui::draw_error_banner};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct App {
    pub state: State,
    pub icons: Icons,
    pub input_manager: FingerManager,
    pub stylet_manager: StyletManager,
    pub x_screen_size: u32,
    pub y_screen_size: u32,
    pub window_state: Arc<Mutex<WindowState>>,
    // pub last_pen_state: Option<PenState>,
    pub ppp: f32,

    pub debug_info: DebugInfo,
    pub nbr_redraw: u32,
    pub errors: Vec<DisplayError>,
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
            state: State::default(),
            icons: icons,
            window_state: Arc::new(Mutex::new(WindowState::default())),
            stylet_manager: StyletManager::default(),
            input_manager: FingerManager::default(),
            x_screen_size: 1,
            y_screen_size: 1,
            // last_pen_state: None,
            // clicks: vec![],
            ppp: 1.,
            debug_info: DebugInfo::default(),
            nbr_redraw: 0,
            errors: vec![],
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
        Self::try_local_save(&app);
        app
    }

    fn should_backup_today(_app: &App) -> bool {
        let today = Local::now().date_naive();
        let last_save = get_last_save();

        println!("Last save {:?}\n Today {:?} ", last_save, today);

        match last_save {
            Some(last) => last != today,
            None => true, // aucune sauvegarde → on sauvegarde
        }
    }
    pub fn try_local_save(app: &App) {
        let should_save = Self::should_backup_today(app);
        println!("Should save: {}", should_save);
        if should_save {
            let res = create_backup_notes_directory();
            if let Err(err) = res {
                println!("{:?}", err);
            }
        }
    }

    pub fn user_opened_project(&mut self, path: PathBuf) {
        println!("Opened");
        println!("path: {:?}", path);
        let res = self
            .state
            .opened_projects
            .load_fastnote_project(path.clone());
        if let Err(err) = res {
            println!("Not able to load path ! {:?}", path);
            // self.push_minute_error(ui, message);
            self.push_unsafe_minute_error(format!(
                "Not able to load path {:?} because of {:?}",
                path, err
            ));
        }
    }

    pub fn user_created_project(&mut self, new_project_modal_window: &NewProjectModalWindow) {
        let dialog = new_project_modal_window;
        let err = self.state.opened_projects.create_blank_project(
            dialog.path.clone().join(dialog.name.clone()),
            dialog.name.clone(),
            dialog.color,
        );
        println!("{:?}", err);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let has_focus = ctx.input(|i| i.focused);
        self.state.app_have_focus = has_focus;
        #[cfg(feature = "debug-input")]
        println!("have focus: {}", has_focus);
        self.stylet_manager
            .manage_events(ctx, &mut self.state, &has_focus);
        // println!("has focus{}", has_focus);
        // if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl) {
        //     println!("Save state");
        // }/
        // if ctx.input(|i| i.key_pressed(egui::Key::L) && i.modifiers.ctrl) {
        //     println!("Load state");
        // }
        // if ctx.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl) {}
        // let window_pos = ctx
        //     .input(|i| i.viewport().outer_rect)
        //     .map(|r| r.min)
        //     .unwrap_or_else(|| {
        //         println!("zero");
        //         egui::Pos2::ZERO
        //     });
        //
        //
        #[cfg(feature = "debug-input")]
        {
            let left_down = ctx.input(|i| i.pointer.primary_down());
            let right_down = ctx.input(|i| i.pointer.secondary_down());
            let left_clicked = ctx.input(|i| i.pointer.primary_clicked());
            let right_clicked = ctx.input(|i| i.pointer.secondary_clicked());

            println!(
                "{} {} {} {}",
                left_down, right_down, left_clicked, right_clicked
            );

        }


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
    pub fn reload_current_project(&mut self) {
        self.state.opened_projects =
            crate::projects::opened_projects::OpenedProjectsManager::default();
    }

    /// Ajoute une erreur qui dure X secondes
    pub fn push_error(&mut self, ui: &egui::Ui, message: impl Into<String>, seconds: f64) {
        self.errors
            .push(DisplayError::new(message, Duration::from_secs_f64(seconds)));
        ui.request_repaint_after_secs(seconds as f32);
    }

    pub fn push_instant_error(&mut self, message: impl Into<String>) {
        self.errors
            .push(DisplayError::new(message, Duration::from_secs_f64(0.)));
    }
    pub fn push_minute_error(&mut self, ui: &egui::Ui, message: impl Into<String>) {
        self.errors
            .push(DisplayError::new(message, Duration::from_secs_f64(60.)));
        ui.request_repaint_after_secs(60.);
    }

    pub fn push_unsafe_minute_error(&mut self, message: impl Into<String>) {
        self.errors
            .push(DisplayError::new(message, Duration::from_secs_f64(60.)));
    }

    /// Nettoie les erreurs expirées
    pub fn cleanup_errors(&mut self) {
        self.errors.retain(|e| e.is_active());
    }

    pub fn open_page(&mut self, path: PathBuf) {
        self.state.gpu_view.top_left = Pos2::ZERO;
        self.state.loaded_page = Some(LoadedPage::new(path));
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);.clone()
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        self.update(ui.ctx(), frame);

        let rect = ui.available_rect_before_wrap();
        let pixel_rect = rect_points_to_pixels(rect, ui.pixels_per_point());
        self.state.top_level_rect_egui = Some(rect);
        self.state.top_level_rect_pix = Some(pixel_rect);

        // println!("viewport {}", ViewportId::ROOT);
        self.debug_info.lines = vec![];
        // ui.ctx().set_cursor_icon(self.state.cursor_icon);
        // println!("{}", self.stylet_manager.stylet.pressed);

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

        //Error
        // I’m not sure about how timed errors would act.
        let cpy = std::mem::take(&mut self.errors);
        for (j, error) in cpy.into_iter().enumerate() {
            draw_error_banner(ui, &error.message, j);
            if error.is_active() {
                self.errors.push(error);
            }
        }

        //Debug
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

        // if self.stylet_manager.stylet.pressed {
        //     ui.ctx().input_mut(|i| {
        //         i.pointer = egui::PointerState::default(); // efface le pointer souris
        //     });

        //     ui.ctx().set_cursor_icon(egui::CursorIcon::None);
        // }
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
