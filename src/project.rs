use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub path: PathBuf,
    pub kind: String,
    pub description: String,
    pub priority: i32,
}

impl Project {
    pub fn new(name: &str, path: &Path) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            kind: String::new(),
            description: String::new(),
            priority: 0,
        }
    }

    pub fn find_project(&mut self, path: &Path) {
        for current_path in path.ancestors() {
            let entries = match fs::read_dir(current_path) {
                Ok(e) => e,
                Err(_) => continue,
            };

            let files: HashSet<_> = entries
                .filter_map(|e| e.ok()?.file_name().to_str().map(String::from))
                .collect();

            if files.is_empty() {
                continue;
            }

            for patterns_map in PROJECT_LANGUAGE_FILES_COLLECT.iter() {
                if let Some((kind, _)) = patterns_map
                    .iter()
                    .find(|(_, p)| p.iter().any(|f| files.contains(*f)))
                {
                    self.path = current_path.to_path_buf();
                    self.kind = kind.to_string();
                    return;
                }
            }
        }
    }
}

static PROJECT_LANGUAGE_FILES_COLLECT: LazyLock<Vec<HashMap<&str, Vec<&str>>>> =
    LazyLock::new(|| {
        let mut total = Vec::new();
        let mut language_map = HashMap::new();
        let mut vc_map = HashMap::new();
        let mut misc_map = HashMap::new();

        language_map.insert(
            "c",
            vec![
                "compile_commands.json",
                "compile_flags.txt",
                "Makefile",
                "configure.ac",
                "configure.in",
                "cscope.out",
                "GTAGS",
                "TAGS",
            ],
        );

        language_map.insert(
            "cpp",
            vec![
                "compile_commands.json",
                "compile_flags.txt",
                "Makefile",
                ".clangd",
                ".ccls-cache",
            ],
        );

        // Python
        language_map.insert(
            "python",
            vec![
                "pyproject.toml",
                "requirements.txt",
                "setup.py",
                "tox.ini",
                ".tox",
                "pyrightconfig.json",
            ],
        );

        // JavaScript/Node.js
        language_map.insert(
            "nodejs",
            vec![
                "package.json",
                "yarn.lock",
                "pnpm-lock.yaml",
                "webpack.config.js",
                "rollup.config.js",
                "vite.config.js",
            ],
        );

        // Go
        language_map.insert("go", vec!["go.mod", "go.sum"]);

        // Rust
        language_map.insert("rust", vec!["Cargo.toml", "Cargo.lock"]);

        // Java
        language_map.insert(
            "java",
            vec![
                "pom.xml",
                "build.gradle",
                "build.gradle.kts",
                ".classpath",
                ".project",
            ],
        );

        // Haskell
        language_map.insert(
            "haskell",
            vec!["stack.yaml", "cabal.config", "package.yaml", "hie-bios"],
        );

        // Dart/Flutter
        language_map.insert("dart", vec!["pubspec.yaml"]);

        // Ruby
        language_map.insert("ruby", vec!["Gemfile", "Gemfile.lock"]);

        // PHP
        language_map.insert("php", vec!["composer.json", "composer.lock"]);

        // Docker
        language_map.insert("docker", vec!["Dockerfile", "docker-compose.yml"]);

        // Elm
        language_map.insert("elm", vec!["elm.json"]);

        // Fortran
        language_map.insert("fortran", vec!["fortls"]);

        // Nix
        language_map.insert("nix", vec!["flake.nix", ".envrc"]);

        // Scala
        language_map.insert("scala", vec!["build.sbt", ".ensime_cache"]);

        // Vue
        language_map.insert("vue", vec!["vue.config.js"]);

        // Godot
        language_map.insert("godot", vec!["project.godot"]);

        vc_map.insert("git", vec![".git", ".gitignore"]);
        vc_map.insert("svn", vec![".svn"]);
        vc_map.insert("mercurial", vec![".hg"]);
        vc_map.insert("bazaar", vec![".bzr"]);
        vc_map.insert("fossil", vec!["_FOSSIL_", ".fslckout"]);
        vc_map.insert("pijul", vec![".pijul"]);

        misc_map.insert("editor", vec![".idea", ".vscode"]);

        // Miscellaneous
        misc_map.insert("ocaml", vec![".merlin"]);
        misc_map.insert("erlang", vec![".eunit"]);
        misc_map.insert("make", vec!["Makefile"]);
        misc_map.insert("metals", vec!["metals.sbt", "build.sc"]);
        misc_map.insert("environment", vec![".env", ".envrc"]);
        misc_map.insert("cache", vec![".cache"]);

        total.push(language_map);
        total.push(vc_map);
        total.push(misc_map);
        total
    });
