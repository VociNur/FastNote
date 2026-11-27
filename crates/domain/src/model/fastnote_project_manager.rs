use std::path::PathBuf;

use eframe::egui::Color32;

use crate::{model::fastnote_project::FastnoteProject, storage::SharedStorage};

#[derive(Clone)]
pub struct OpenedProjectsManager {
    pub projects: Vec<FastnoteProject>,
    storage: SharedStorage,
}

// impl Default for OpenedProjectsManager {
//     fn default() -> OpenedProjectsManager {
//         let mut s = OpenedProjectsManager { projects: vec![] };
//         let res_def = OpenedProjectsManager::load_opened_project_manager(&mut s);
//         if res_def.is_ok() {
//             s
//         } else {
//             println!("Unable to load default opened projects");
//             OpenedProjectsManager::new_blank()
//         }
//     }
// }

impl OpenedProjectsManager {
    pub fn load(storage: SharedStorage) -> OpenedProjectsManager {
        let mut s = OpenedProjectsManager {
            storage: storage.clone(),
            projects: vec![],
        };
        // let res_def = OpenedProjectsManager::load_opened_project_manager(&mut s);
        let vec_from_file_opt = storage.load_open_projects_file();
        if let Ok(vec_from_file) = vec_from_file_opt {
            for p in vec_from_file {
                // self.load_fastnote_project(p)?;
                let project_res = storage.load_project(&p);
                if let Ok(project) = project_res {
                    s.projects.push(project);
                }
            }
        } else {
            println!("Unable to load default opened projects file");
        }
        s
    }

    //devrait save ses fils ?
    pub fn save(&mut self) {
        let data: Vec<PathBuf> = self
            .projects
            .iter()
            .map(|p| p.path.clone())
            .collect::<Vec<PathBuf>>();
        self.storage.save_open_projects_file(&data);
    }

    pub fn create_blank_project(
        self: &mut Self,
        path: PathBuf,
        name: String,
        color: Color32,
    ) -> anyhow::Result<()> {
        self.projects.push(FastnoteProject::create_blank(
            self.storage.clone(),
            path,
            name,
            color,
        )?);
        // self.save_opened_project_manager();
        //
        Ok(())
    }

    // pub fn load_fastnote_project(self: &mut Self, path: PathBuf) -> anyhow::Result<()> {
    //     // let project = FastnoteProject::load(path)?;
    //     let project = self.storage.load_project(&path)?;
    //     self.projects.push(project);
    //     // self.save_opened_project_manager();
    //     Ok(())
    // }
    //
    // pub fn unload_fastnote_project_from_path(self: &mut Self, path: PathBuf) {
    //     self.projects.retain(|f| {
    //         f.path.canonicalize().is_ok()
    //             && path.canonicalize().is_ok()
    //             && f.path.canonicalize().unwrap() != path.canonicalize().unwrap()
    //     });
    //     self.save();
    // }

    // #[deprecated(since = "0.2.0", note = "use sm instead")]
    // pub fn load_opened_project_manager(&mut self) -> anyhow::Result<()> {
    // }

    // pub fn save_opened_project_manager(&mut self) {
    //     let path_opened_projects_data = get_working_path().join(OPENED_PROJECTS_FILE);
    //     let data: Vec<PathBuf> = self
    //         .projects
    //         .iter()
    //         .map(|p| p.path.clone())
    //         .collect::<Vec<PathBuf>>();
    //     let res_json = serde_json::to_string_pretty(&data);
    //     if res_json.is_err() {
    //         println!("[OpenedProject->Save] Could not cast to json");
    //         return;
    //     }
    //     let json = res_json.unwrap();
    //     save_persistent_data(path_opened_projects_data, &json);
    // }

    pub fn new_blank(storage: SharedStorage) -> Self {
        Self {
            storage,
            projects: vec![],
        }
    }
}
