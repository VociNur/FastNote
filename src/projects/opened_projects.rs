use std::path::PathBuf;

use eframe::egui::Color32;

use crate::{
    get_working_path, load_persistent_data,
    paths::OPENED_PROJECTS_FILE,
    projects::fastnote_project::FastnoteProject,
    save_persistent_data,
};

#[derive(Clone)]
pub struct OpenedProjectsManager {
    pub projects: Vec<FastnoteProject>,
}

impl Default for OpenedProjectsManager {
    fn default() -> OpenedProjectsManager {
        let mut s = OpenedProjectsManager { projects: vec![] };
        let res_def = OpenedProjectsManager::load_opened_project_manager(&mut s);
        if res_def.is_ok() {
            s
        } else {
            println!("Unable to load default opened projects");
            OpenedProjectsManager::new_blank()
        }
    }
}

impl OpenedProjectsManager {
    pub fn create_blank_project(
        self: &mut Self,
        path: PathBuf,
        name: String,
        color: Color32,
    ) -> anyhow::Result<()> {
        self.projects
            .push(FastnoteProject::create_blank(path, name, color)?);
        self.save_opened_project_manager();
        Ok(())
    }

    pub fn load_fastnote_project(self: &mut Self, path: PathBuf) -> anyhow::Result<()> {
        let project = FastnoteProject::load(path)?;
        self.projects.push(project);
        self.save_opened_project_manager();
        Ok(())
    }
    pub fn unload_fastnote_project_from_path(self: &mut Self, path: PathBuf) {
        self.projects.retain(|f| {
            f.path.canonicalize().is_ok()
                && path.canonicalize().is_ok()
                && f.path.canonicalize().unwrap() != path.canonicalize().unwrap()
        });
        self.save_opened_project_manager();
    }

    pub fn load_opened_project_manager(&mut self) -> anyhow::Result<()> {
        let path_opened_projects_data = get_working_path().join(OPENED_PROJECTS_FILE);

        let json = load_persistent_data(path_opened_projects_data)?;
        // println!("json {json}");
        let cast: Vec<PathBuf> = serde_json::from_str(&json)?;
        // println!("cast");
        for path in cast {
            self.load_fastnote_project(path)?;
        }
        Ok(())
    }

    pub fn save_opened_project_manager(&mut self) {
        let path_opened_projects_data = get_working_path().join(OPENED_PROJECTS_FILE);
        let data: Vec<PathBuf> = self
            .projects
            .iter()
            .map(|p| p.path.clone())
            .collect::<Vec<PathBuf>>();
        let res_json = serde_json::to_string_pretty(&data);
        if res_json.is_err() {
            println!("[OpenedProject->Save] Could not cast to json");
            return;
        }
        let json = res_json.unwrap();
        save_persistent_data(path_opened_projects_data, &json);
    }

    pub fn new_blank() -> Self {
        Self { projects: vec![] }
    }
}
