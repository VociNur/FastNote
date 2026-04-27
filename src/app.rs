use std::path::{Path, PathBuf};

use egui::Color32;

use crate::{icons::Icons, state::State, ui::ui::draw_gui, user_project::UserProject};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct App {
    pub state: State,
    pub icons: Icons,
}

impl App {
    fn default(icons: Icons) -> Self {
        Self {
            state: State::new(),
            icons: icons,
        }
    }
}

impl App {
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
        App::default(icons)
    }
    pub fn open_project(&mut self, path: PathBuf){
        self.state.opened_projects.push(UserProject::new(path, Color32::BLACK));
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
    }
}

