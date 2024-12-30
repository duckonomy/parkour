use crate::project::Project;
use bincode;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::{canonicalize, File},
    io::{self, Read, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectState {
    pub projects: HashMap<PathBuf, Project>,
    pub blacklist: HashSet<PathBuf>,
}

fn get_state_file_path() -> io::Result<PathBuf> {
    let home = env::var("HOME").map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))?;

    match env::consts::OS {
        "macos" => Ok(PathBuf::from(home).join("Library/Application Support/pk/pk.db")),
        "linux" => Ok(PathBuf::from(home).join(".local/state/pk/pk.db")),
        _ => Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported OS")),
    }
}

impl ProjectState {
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            blacklist: HashSet::new(),
        }
    }

    pub fn init(&mut self) {
        match self.load_state() {
            Ok(state) => {
                self.projects = state.projects;
                self.blacklist = state.blacklist;
            }
            Err(_) => {
                // If we can't load the state, save the empty state
                self.save_state().unwrap_or_default();
            }
        }
    }

    pub fn list_projects(&self) {
        let mut projects: Vec<&Project> = self
            .projects
            .values()
            .filter(|p| !self.blacklist.contains(&p.path))
            .collect();

        projects.sort_by(|a, b| b.priority.cmp(&a.priority));

        for project in projects {
            println!("{}", project.path.display());
        }
    }

    fn load_state(&self) -> io::Result<ProjectState> {
        let state_file_path = get_state_file_path()?;

        if !state_file_path.exists() {
            return Ok(ProjectState {
                projects: HashMap::new(),
                blacklist: HashSet::new(),
            });
        }

        let mut file = File::options().read(true).open(state_file_path)?;

        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;

        if buf.is_empty() {
            return Ok(ProjectState {
                projects: HashMap::new(),
                blacklist: HashSet::new(),
            });
        }

        bincode::deserialize(&buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    fn save_state(&self) -> io::Result<()> {
        let state_file_path = get_state_file_path()?;

        if let Some(parent) = state_file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let encoded =
            bincode::serialize(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let mut file = File::create(&state_file_path)?;
        file.write_all(&encoded)?;

        Ok(())
    }

    pub fn add_project(&mut self, path: &Path) {
        let norm_path = match canonicalize(path) {
            Ok(p) => p,
            Err(_) => {
                println!("Invalid path: {}", path.display());
                return;
            }
        };

        if self.blacklist.contains(&norm_path) {
            println!("Path is blacklisted");
            return;
        }

        if self.projects.contains_key(&norm_path) {
            println!("Project already exists");
            return;
        }

        let project = Project::new(
            &norm_path.file_name().unwrap_or_default().to_string_lossy(),
            &norm_path,
        );

        // self.projects.insert(path_str, project);
        self.projects.insert(norm_path, project);
        self.save_state().unwrap_or_else(|_| {
            println!("Failed to save state");
        });
    }

    pub fn remove_project(&mut self, path: &Path) {
        let norm_path = match canonicalize(path) {
            Ok(p) => p,
            Err(_) => {
                println!("Invalid path: {}", path.display());
                return;
            }
        };

        if self.projects.remove(&norm_path).is_none() {
            println!("Project not found");
            return;
        }

        self.save_state().unwrap_or_else(|_| {
            println!("Failed to save state");
        });
    }

    pub fn show_blacklist(&self) {
        for blocked in self.blacklist.iter() {
            println!("{}", blocked.display());
        }
    }

    pub fn get_project(&mut self, path: &Path) -> io::Result<Option<Project>> {
        let norm_path = canonicalize(path).unwrap();

        if let Some(project) = self.projects.remove(&norm_path) {
            self.increment_project_priority(&norm_path);
            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    pub fn manage_blacklist(&mut self, path: &Path, add: bool) {
        let norm_path = match canonicalize(path) {
            Ok(p) => p,
            Err(_) => {
                println!("Invalid path: {}", path.display());
                return;
            }
        };

        if add {
            self.blacklist.insert(norm_path);
        } else {
            self.blacklist.remove(&norm_path);
        }

        self.save_state().unwrap_or_else(|_| {
            println!("Failed to save state");
        });
    }

    fn increment_project_priority(&mut self, path: &Path) {
        if let Some(project) = self.projects.get_mut(path) {
            project.priority += 1;
            self.save_state().unwrap();
            return;
        }
    }
}
