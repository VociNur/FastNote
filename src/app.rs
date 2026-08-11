use crate::stylet::stylet_manager::{
    AxisEventState, ButtonEventState, ProximityEventState, StyletEvent, TipEventState,
};
use eframe::egui::{self, Pos2, Rect};
use std::sync::{Arc, Mutex};
use std::{
    fs::OpenOptions,
    os::unix::{
        fs::OpenOptionsExt,
        io::{AsRawFd, OwnedFd},
    },
    path::Path,
};

use input::{
    event::{
        tablet_tool::{TabletToolEvent, TabletToolEventTrait, TabletToolType},
        Event,
    },
    Libinput, LibinputInterface,
};
use std::path::PathBuf;

use crate::icons::Icons;
use crate::stylet::stylet_manager::StyletManager;
use crate::ui::ui::draw_gui;
use crate::{
    edition::open_edition_mode, input_manager::InputManager, projects::user_file::UserFile,
    state::State,
};
#[derive(Default, Clone)]
pub struct WindowState {
    pub pos: egui::Pos2,
    // pub ppp: f32, // pixels_per_point
}
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
    //poru stylet input
    libinput: Option<Libinput>,
    libinput_fd: Option<i32>,
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
struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read(true)
            .write(true)
            .open(path)
            //.map(|f| unsafe { OwnedFd::from_raw_fd(f.into_raw_fd()) })
            .map(OwnedFd::from)
            .map_err(|e| e.raw_os_error().unwrap_or(-1))
    }

    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(fd);
    }
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
            libinput: None,
            libinput_fd: None,
        }
    }
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, icons: Icons, width: u32, height: u32) -> Self {
        let mut app = App::default(icons);
        let wgpu_state = cc.wgpu_render_state.as_ref().unwrap();
        println!("msaa samples: {:?}", wgpu_state.target_format);
        app.x_screen_size = width;
        app.y_screen_size = height;
        // spawn_pen_thread(
        //     Arc::clone(&app.window_state),
        //     Arc::clone(&app.stylet_manager.events),
        //     width,
        //     height,
        // );

        //stylet input
        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();

        app.libinput_fd = Some(input.as_raw_fd());
        app.libinput = Some(input);
        //
        // end stylet input
        app
    }

    #[allow(unsafe_code)]
    fn read_stylet_input(&mut self, window_pos: Pos2, width: u32, height: u32) {
        if let (Some(input), Some(fd)) = (&mut self.libinput, self.libinput_fd) {
            let mut pollfd = libc::pollfd {
                fd,
                events: libc::POLLIN,
                revents: 0,
            };

            unsafe {
                libc::poll(&mut pollfd, 1, 0); // timeout = 0 → non bloquant
            }

            let mut batch = vec![];
            // 🚨 IMPORTANT : ne dispatch QUE si poll dit qu’il y a des events
            if pollfd.revents & libc::POLLIN != 0 {
                input.dispatch().unwrap();

                for event in input {
                    println!("{:?}", event);
                    if let Event::Tablet(tablet_event) = event {
                        // if !matches!(tablet_event, TabletToolEvent::Axis(_)){
                        //     println!("tablet_event: {tablet_event:?}");

                        // }
                        // println!("{:?}", tablet_event.tool().tool_type());
                        let pos = egui::pos2(
                            tablet_event.x_transformed(width) as f32 - window_pos.x * 2.,
                            tablet_event.y_transformed(height) as f32 - window_pos.y * 2., //todo ppp
                        );
                        let tooltype = tablet_event.tool().tool_type().unwrap_or_else(|| {
                            println!("No tool type, default: pen");
                            TabletToolType::Pen
                        });
                        match tablet_event {
                            TabletToolEvent::Axis(axis_event) => {
                                batch.push(StyletEvent::Axis(AxisEventState::new(
                                    pos,
                                    axis_event.pressure(),
                                    axis_event.distance(),
                                    axis_event.tilt_x(),
                                    axis_event.tilt_y(),
                                    tooltype,
                                )));
                            }
                            TabletToolEvent::Tip(tip_event) => {
                                batch.push(StyletEvent::Tip(TipEventState::new(
                                    pos,
                                    tip_event.pressure(),
                                    tip_event.distance(),
                                    tip_event.tilt_x(),
                                    tip_event.tilt_y(),
                                    tip_event.tip_state(),
                                    tooltype,
                                )));
                            }
                            TabletToolEvent::Proximity(proximity_event) => {
                                batch.push(StyletEvent::Proximity(ProximityEventState::new(
                                    pos,
                                    proximity_event.pressure(),
                                    proximity_event.distance(),
                                    proximity_event.tilt_x(),
                                    proximity_event.tilt_y(),
                                    proximity_event.proximity_state(),
                                    tooltype,
                                )));
                            }
                            TabletToolEvent::Button(button_event) => {
                                batch.push(StyletEvent::Button(ButtonEventState::new(
                                    button_event.button(),
                                    button_event.button_state(),
                                    tooltype,
                                )));
                            }
                            _ => todo!(),
                        }
                    }

                    //attention, bouton erase considéré à part
                }
            }
            let change = batch.len() != 0;
            self.stylet_manager.events.lock().unwrap().extend(batch);
            if change {
                // ctx.request_repaint(); // indispensable pour egui
            }
        }
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
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
        let window_pos = ctx
            .input(|i| i.viewport().inner_rect)
            .map(|r| r.min)
            .unwrap_or_else(|| {
                println!("zero");
                egui::Pos2::ZERO
            });
        // let ppp = ctx.pixels_per_point();

        {
            let mut w = self.window_state.lock().unwrap();
            w.pos = window_pos;
            // w.ppp = ppp;
        };

        ctx.input(|i| {
            for event in &i.events {
                // println!("Event : {:?}", event);
                self.input_manager
                    .manage_events(&mut self.state, event.clone(), self.ppp);
            }
        });
        self.read_stylet_input(window_pos, 0, 0);
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
