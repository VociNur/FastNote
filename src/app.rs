use std::{path::{self, Path, PathBuf}, sync::{Arc, Mutex}};

use egui::{Color32};

use crate::{edition::open_edition_mode, icons::Icons, state::State, stylet::spawn_pen_thread, ui::ui::draw_gui, user_project::UserProject };

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct App {
    pub state: State,
    pub icons: Icons,
    pub window_state: Arc<Mutex<WindowState>>,
    pub pen_state: Arc<Mutex<PenState>>,
    pub last_pen_state: Option<PenState>,
}

pub fn load_state() -> State {
    let path = save_path();
    if path.exists() {
        let json = std::fs::read_to_string(path).unwrap_or_default();
        serde_json::from_str(&json).unwrap_or_default()
    } else {
       State::default()

    }
}

fn save_path() -> std::path::PathBuf {
    let path = dirs::data_dir()                    // C:\Users\...\AppData\Roaming  (Windows)
                                        // ~/.local/share               (Linux)
                                        // ~/Library/Application Support (Mac)
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("fastnote")
        .join("save.json");
    println!("Path : {:?} {:?}", &path, path::absolute(&path));
    path
}

#[derive(Default, Clone)]
pub struct PenState {
    pub pos: egui::Pos2,
    pub pressed: bool,
}

#[derive(Default, Clone)]
pub struct WindowState {
    pub pos: egui::Pos2,
    pub ppp: f32,  // pixels_per_point
}
impl App {
    fn default(icons: Icons) -> Self {
        Self {
            state: load_state(),
            icons: icons,
            window_state: Arc::new(Mutex::new(WindowState::default())),
            pen_state: Arc::new(Mutex::new(PenState::default())),
            last_pen_state: None,
            
            
        }
    }
    // Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>, icons:Icons) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
            // eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        // } else {
            // Default::default()
        // }
        let app = App::default(icons);

        spawn_pen_thread(Arc::clone(&app.pen_state), Arc::clone(&app.window_state));
        app
    }

    pub fn open_project(&mut self, path: PathBuf){
        self.state.opened_projects.push(UserProject::new(path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unnamed").to_owned(), path.clone(), Color32::BLACK));
        println!("Opened {:?}", path);
    }

    pub fn save_state(&mut self) -> anyhow::Result<()>{

        let path = save_path();
        let json = serde_json::to_string_pretty(&self.state)?;
        let tmp_path = path.with_extension("tmp");
        std::fs::write(&tmp_path, json)?;
    
        // 2. Renommer atomiquement → remplace l'ancien fichier d'un coup
        std::fs::rename(&tmp_path, path)?;
        
        Ok(())
    }
    
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();

        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = egui::Color32::from_rgb(77, 79, 83);
        ctx.set_visuals(visuals);        

        draw_gui(ui,self);
        if self.state.edition_open {
            open_edition_mode(&mut self.state.theme, ui.ctx(), &mut self.state.edition_open);
        }
    }
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        if ctx.input(|i| i.key_pressed(egui::Key::S) && i.modifiers.ctrl){
            let _ = self.save_state();
            println!("Save state");
        }
        if ctx.input(|i| i.key_pressed(egui::Key::L) && i.modifiers.ctrl){
            self.state = load_state();
            println!("Load state");
        }
        if ctx.input(|i| i.key_pressed(egui::Key::P) && i.modifiers.ctrl){
            println!("Load state");
            let json = serde_json::to_string_pretty(&self.state).unwrap_or_default();
            println!("{}", json);
        }
        let window_pos = ctx.input(|i| i.viewport().outer_rect)
            .map(|r| r.min)
            .unwrap_or_else(|| {println!("zero"); egui::Pos2::ZERO});
        // let ppp = ctx.pixels_per_point();
        
        {
            let mut w = self.window_state.lock().unwrap();
            w.pos = window_pos;
            // w.ppp = ppp;
        };

        
        let pen_state = self.pen_state.lock().unwrap().clone();
        let opt_last_pen_state = self.last_pen_state.take();
        let mut events = vec![egui::Event::PointerMoved(pen_state.pos)];

        if let Some(last_pen_state) = opt_last_pen_state && pen_state.pressed != last_pen_state.pressed {
            if pen_state.pressed {
                println!("clic à ({:.0}, {:.0})", pen_state.pos.x, pen_state.pos.y);
            }
            events.push(egui::Event::PointerButton {
                pos: pen_state.pos,
                button: egui::PointerButton::Primary,
                pressed: pen_state.pressed,
                modifiers: egui::Modifiers::default(),
            });
        }
        self.last_pen_state = Some(pen_state);

        ctx.input_mut(|i| i.events.extend(events));
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

