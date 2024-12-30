mod project;
mod state;

use clap::{Parser, Subcommand};
use state::ProjectState;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "pk")]
#[command(about = "Project root finder")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to check (for default command)
    path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add current or specified path to projects list
    #[command(alias = "a")]
    Add {
        /// Path to add (optional, uses current directory if not specified)
        path: Option<PathBuf>,
    },

    /// List all saved projects
    #[command(alias = "l")]
    List,

    /// Remove a project from the list
    #[command(alias = "r")]
    Remove {
        /// Path to remove (optional, uses current directory if not specified)
        path: Option<PathBuf>,
    },

    #[command(subcommand)]
    Blacklist(BlacklistCommands),
}

#[derive(Subcommand)]
enum BlacklistCommands {
    /// Add a path to blacklist
    Add {
        /// Path to blacklist (optional, uses current directory if not specified)
        path: Option<PathBuf>,
    },
    /// Remove a path from blacklist
    Remove {
        /// Path to remove from blacklist (optional, uses current directory if not specified)
        path: Option<PathBuf>,
    },
    /// List all blacklisted paths
    List,
}

fn find_project_root(path: &PathBuf) -> project::Project {
    let root_path = if path.is_file() {
        path.parent().unwrap_or(Path::new("."))
    } else {
        path.as_path()
    };

    let mut project =
        project::Project::new(root_path.display().to_string(), root_path.to_path_buf());

    project.find_project(root_path);
    project
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { path }) => {
            let mut project_state = ProjectState::new();
            project_state.init();

            let path = path
                .clone()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

            project_state.add_project(&path);
        }

        Some(Commands::List) => {
            let mut project_state = ProjectState::new();
            project_state.init();
            project_state.list_projects();
        }

        Some(Commands::Remove { path }) => {
            let mut project_state = ProjectState::new();
            project_state.init();

            let path = path
                .clone()
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

            project_state.remove_project(&path);
        }

        Some(Commands::Blacklist(blacklist_command)) => {
            let mut project_state = ProjectState::new();
            project_state.init();

            match blacklist_command {
                BlacklistCommands::Add { path } => {
                    let path = path
                        .clone()
                        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
                    project_state.manage_blacklist(&path, true);
                }
                BlacklistCommands::Remove { path } => {
                    let path = path
                        .clone()
                        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
                    project_state.manage_blacklist(&path, false);
                }
                BlacklistCommands::List => {
                    project_state.show_blacklist();
                }
            }
        }

        None => {
            let mut project_state = ProjectState::new();
            project_state.init();

            let path = cli
                .path
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());

            let current_project: project::Project;

            if let Ok(Some(project)) = project_state.get_project(&path) {
                current_project = project;
            } else {
                current_project = find_project_root(&path);
            }

            println!("{}", current_project.path.display());
        }
    }

    Ok(())
}
