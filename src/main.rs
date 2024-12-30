mod project;
mod state;

use clap::{Parser, Subcommand};
use state::ProjectState;
use std::error::Error;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "pk")]
#[command(about = "Project Manager")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    path: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "a")]
    Add { path: Option<PathBuf> },

    #[command(alias = "l")]
    List,

    #[command(alias = "r")]
    Remove { path: Option<PathBuf> },

    #[command(subcommand)]
    Blacklist(BlacklistCommands),
}

#[derive(Subcommand)]
enum BlacklistCommands {
    Add { path: Option<PathBuf> },
    Remove { path: Option<PathBuf> },
    List,
}

fn find_project_root(path: &PathBuf) -> project::Project {
    let root_path = if path.is_file() {
        path.parent().unwrap_or(Path::new("."))
    } else {
        path.as_path()
    };

    let mut project = project::Project::new(
        &root_path.file_name().unwrap_or_default().to_string_lossy(),
        root_path,
    );

    project.find_project(root_path);
    project
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add { path }) => {
            let mut project_state = ProjectState::new();
            project_state.init();

            if let Some(path) = path {
                project_state.add_project(path);
            }
        }

        Some(Commands::List) => {
            let mut project_state = ProjectState::new();
            project_state.init();
            project_state.list_projects();
        }

        Some(Commands::Remove { path }) => {
            let mut project_state = ProjectState::new();
            project_state.init();
            if let Some(path) = path {
                project_state.remove_project(path);
            }
        }

        Some(Commands::Blacklist(blacklist_command)) => {
            let mut project_state = ProjectState::new();
            project_state.init();

            match blacklist_command {
                BlacklistCommands::Add { path } => {
                    if let Some(path) = path {
                        project_state.manage_blacklist(path, true);
                    }
                }
                BlacklistCommands::Remove { path } => {
                    if let Some(path) = path {
                        project_state.manage_blacklist(path, false);
                    }
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
