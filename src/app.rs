use std::{path::{self, PathBuf}, sync::{Arc, Mutex}};
use eframe::egui::{self, Rect, Vec2};
use egui::Color32;

use crate::{edition::open_edition_mode, strokes::StrokePoint, user_file::UserFile};
use crate::user_project::UserProject;
use crate::ui::ui::draw_gui;
use crate::stylet::stylet_inputs::spawn_pen_thread;
use crate::state::State;
use crate::icons::Icons;
use crate::stylet::stylet_manager::StyletManager;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct App {
    pub app_have_focus: bool,
    pub state: State,
    pub icons: Icons,
    pub stylet_manager: StyletManager,
    pub gpu_rect: Option<Rect>,
    pub window_state: Arc<Mutex<WindowState>>,
    // pub last_pen_state: Option<PenState>,

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
    pub pressure: f64,
}

#[derive(Default, Clone)]
pub struct WindowState {
    pub pos: egui::Pos2,
    pub ppp: f32,  // pixels_per_point
}
impl App {
    fn default(icons: Icons) -> Self {
        Self {
            app_have_focus: false,
            state: load_state(),
            icons: icons,
            gpu_rect: None,
            window_state: Arc::new(Mutex::new(WindowState::default())),
            stylet_manager: StyletManager::default(),
            // last_pen_state: None,
            // clicks: vec![],
            
            
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

        spawn_pen_thread(Arc::clone(&app.window_state), Arc::clone(&app.stylet_manager.events));
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
    pub fn open_file(&mut self, file_path: PathBuf){

        
        let json = std::fs::read_to_string(&file_path).unwrap_or_default();
        let user_file: UserFile = serde_json::from_str(&json).unwrap_or_default();
        self.state.current_file = Some(user_file);
    }
    
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
         let has_focus = ctx.input(|i| i.focused);
         self.app_have_focus = has_focus;

         self.stylet_manager.manage_events(&mut self.state, &has_focus, &self.gpu_rect);
         // println!("has focus{}", has_focus);
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
        let window_pos = ctx.input(|i| i.viewport().inner_rect)
            .map(|r| r.min)
            .unwrap_or_else(|| {println!("zero"); egui::Pos2::ZERO});
        // let ppp = ctx.pixels_per_point();
        
        {
            let mut w = self.window_state.lock().unwrap();
            w.pos = window_pos;
            // w.ppp = ppp;
        };

        ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::PointerMoved(pos) => {
                        // self.clicks.push(pos.to_vec2() - self.gpu_rect.unwrap().min.to_vec2());
                     }
                    // egui::Event::PointerButton { pos, button, pressed, .. } => { ... }
                    // egui::Event::Scroll(delta) => { ... }
                    _ => {}
                }
            }
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


