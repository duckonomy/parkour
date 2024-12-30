use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    // pub path: String,
    pub path: PathBuf,
    pub kind: String,
    pub description: String,
    pub priority: i32,
}

impl Project {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self {
            name,
            path,
            kind: String::new(),
            description: String::new(),
            priority: 0,
        }
    }

    pub fn find_project(&mut self, path: &Path) {
        for current_path in path.ancestors() {
            let entries = match fs::read_dir(current_path) {
                Ok(entries) => entries.filter_map(|entry| entry.ok()).collect::<Vec<_>>(),
                Err(_) => {
                    eprintln!("Failed to read directory: {:?}", current_path);
                    break;
                }
            };

            if let Some((kind, _)) = PROJECT_LANGUAGE_FILES_COLLECT
                .iter()
                .flat_map(|hm| hm.iter())
                .find(|(_, patterns)| {
                    entries.iter().any(|entry| {
                        // patterns.contains(&entry.file_name().to_string_lossy().to_string())
                        patterns.contains(&entry.file_name().to_string_lossy().to_string())
                    })
                })
            {
                self.path = current_path.to_path_buf();
                self.kind = kind.to_string();
                return;
            }
        }
    }
}

static PROJECT_LANGUAGE_FILES_COLLECT: LazyLock<Vec<HashMap<String, Vec<String>>>> =
    LazyLock::new(|| {
        let mut total = Vec::new();
        let mut language_map = HashMap::new();
        let mut vc_map = HashMap::new();
        let mut misc_map = HashMap::new();

        language_map.insert(
            "c".to_string(),
            vec![
                "compile_commands.json".to_string(),
                "compile_flags.txt".to_string(),
                "Makefile".to_string(),
                "configure.ac".to_string(),
                "configure.in".to_string(),
                "cscope.out".to_string(),
                "GTAGS".to_string(),
                "TAGS".to_string(),
            ],
        );

        language_map.insert(
            "cpp".to_string(),
            vec![
                "compile_commands.json".to_string(),
                "compile_flags.txt".to_string(),
                "Makefile".to_string(),
                ".clangd".to_string(),
                ".ccls-cache".to_string(),
            ],
        );

        // Python
        language_map.insert(
            "python".to_string(),
            vec![
                "pyproject.toml".to_string(),
                "requirements.txt".to_string(),
                "setup.py".to_string(),
                "tox.ini".to_string(),
                ".tox".to_string(),
                "pyrightconfig.json".to_string(),
            ],
        );

        // JavaScript/Node.js
        language_map.insert(
            "nodejs".to_string(),
            vec![
                "package.json".to_string(),
                "yarn.lock".to_string(),
                "pnpm-lock.yaml".to_string(),
                "webpack.config.js".to_string(),
                "rollup.config.js".to_string(),
                "vite.config.js".to_string(),
            ],
        );

        // Go
        language_map.insert(
            "go".to_string(),
            vec!["go.mod".to_string(), "go.sum".to_string()],
        );

        // Rust
        language_map.insert(
            "rust".to_string(),
            vec!["Cargo.toml".to_string(), "Cargo.lock".to_string()],
        );

        // Java
        language_map.insert(
            "java".to_string(),
            vec![
                "pom.xml".to_string(),
                "build.gradle".to_string(),
                "build.gradle.kts".to_string(),
                ".classpath".to_string(),
                ".project".to_string(),
            ],
        );

        // Haskell
        language_map.insert(
            "haskell".to_string(),
            vec![
                "stack.yaml".to_string(),
                "cabal.config".to_string(),
                "package.yaml".to_string(),
                "hie-bios".to_string(),
            ],
        );

        // Dart/Flutter
        language_map.insert("dart".to_string(), vec!["pubspec.yaml".to_string()]);

        // Ruby
        language_map.insert(
            "ruby".to_string(),
            vec!["Gemfile".to_string(), "Gemfile.lock".to_string()],
        );

        // PHP
        language_map.insert(
            "php".to_string(),
            vec!["composer.json".to_string(), "composer.lock".to_string()],
        );

        // Docker
        language_map.insert(
            "docker".to_string(),
            vec!["Dockerfile".to_string(), "docker-compose.yml".to_string()],
        );

        // Elm
        language_map.insert("elm".to_string(), vec!["elm.json".to_string()]);

        // Fortran
        language_map.insert("fortran".to_string(), vec!["fortls".to_string()]);

        // Nix
        language_map.insert(
            "nix".to_string(),
            vec!["flake.nix".to_string(), ".envrc".to_string()],
        );

        // Scala
        language_map.insert(
            "scala".to_string(),
            vec!["build.sbt".to_string(), ".ensime_cache".to_string()],
        );

        // Vue
        language_map.insert("vue".to_string(), vec!["vue.config.js".to_string()]);

        // Godot
        language_map.insert("godot".into(), vec!["project.godot".into()]);

        vc_map.insert(
            "git".to_string(),
            vec![".git".to_string(), ".gitignore".to_string()],
        );
        vc_map.insert("svn".to_string(), vec![".svn".to_string()]);
        vc_map.insert("mercurial".to_string(), vec![".hg".to_string()]);
        vc_map.insert("bazaar".to_string(), vec![".bzr".to_string()]);
        vc_map.insert(
            "fossil".to_string(),
            vec!["_FOSSIL_".to_string(), ".fslckout".to_string()],
        );
        vc_map.insert("pijul".to_string(), vec![".pijul".to_string()]);

        misc_map.insert(
            "editor".to_string(),
            vec![".idea".to_string(), ".vscode".to_string()],
        );

        // Miscellaneous
        misc_map.insert("ocaml".to_string(), vec![".merlin".to_string()]);
        misc_map.insert("erlang".to_string(), vec![".eunit".to_string()]);
        misc_map.insert("make".to_string(), vec!["Makefile".to_string()]);
        misc_map.insert(
            "metals".to_string(),
            vec!["metals.sbt".to_string(), "build.sc".to_string()],
        );
        misc_map.insert(
            "environment".to_string(),
            vec![".env".to_string(), ".envrc".to_string()],
        );
        misc_map.insert("cache".to_string(), vec![".cache".to_string()]);

        total.push(language_map);
        total.push(vc_map);
        total.push(misc_map);
        total
    });
