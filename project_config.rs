// src/project_config.rs - Project configuration and initialization

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub root: String,
    pub languages: Vec<String>,
    pub build_command: Option<String>,
    pub run_command: Option<String>,
    pub test_command: Option<String>,
    pub excluded_paths: Vec<String>,
    pub auto_format: bool,
    pub auto_lint: bool,
}

impl Default for ProjectConfig {
    fn default() -> Self {
        Self {
            name: "Code Project".to_string(),
            root: ".".to_string(),
            languages: vec![
                "rust".to_string(),
                "javascript".to_string(),
                "typescript".to_string(),
            ],
            build_command: None,
            run_command: None,
            test_command: None,
            excluded_paths: vec![
                "node_modules".to_string(),
                ".git".to_string(),
                "target".to_string(),
                "dist".to_string(),
            ],
            auto_format: true,
            auto_lint: false,
        }
    }
}

pub struct ProjectBuilder {
    config: ProjectConfig,
}

impl ProjectBuilder {
    pub fn new(name: String) -> Self {
        Self {
            config: ProjectConfig {
                name,
                ..Default::default()
            },
        }
    }

    pub fn with_root(mut self, root: String) -> Self {
        self.config.root = root;
        self
    }

    pub fn with_languages(mut self, languages: Vec<String>) -> Self {
        self.config.languages = languages;
        self
    }

    pub fn with_build_command(mut self, command: String) -> Self {
        self.config.build_command = Some(command);
        self
    }

    pub fn with_run_command(mut self, command: String) -> Self {
        self.config.run_command = Some(command);
        self
    }

    pub fn with_test_command(mut self, command: String) -> Self {
        self.config.test_command = Some(command);
        self
    }

    pub fn with_excluded_paths(mut self, paths: Vec<String>) -> Self {
        self.config.excluded_paths = paths;
        self
    }

    pub fn with_auto_format(mut self, enabled: bool) -> Self {
        self.config.auto_format = enabled;
        self
    }

    pub fn with_auto_lint(mut self, enabled: bool) -> Self {
        self.config.auto_lint = enabled;
        self
    }

    pub fn build(self) -> ProjectConfig {
        self.config
    }
}

/// Project template for common project types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    pub name: String,
    pub description: String,
    pub languages: Vec<String>,
    pub files: Vec<TemplateFile>,
    pub commands: ProjectCommands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateFile {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCommands {
    pub build: Option<String>,
    pub run: Option<String>,
    pub test: Option<String>,
    pub dev: Option<String>,
}

pub struct TemplateManager;

impl TemplateManager {
    pub fn rust_project() -> ProjectTemplate {
        ProjectTemplate {
            name: "Rust Project".to_string(),
            description: "A new Rust project".to_string(),
            languages: vec!["rust".to_string()],
            files: vec![],
            commands: ProjectCommands {
                build: Some("cargo build".to_string()),
                run: Some("cargo run".to_string()),
                test: Some("cargo test".to_string()),
                dev: Some("cargo run".to_string()),
            },
        }
    }

    pub fn web_project() -> ProjectTemplate {
        ProjectTemplate {
            name: "Web Project".to_string(),
            description: "A new web project".to_string(),
            languages: vec![
                "javascript".to_string(),
                "typescript".to_string(),
                "css".to_string(),
                "html".to_string(),
            ],
            files: vec![],
            commands: ProjectCommands {
                build: Some("npm run build".to_string()),
                run: Some("npm start".to_string()),
                test: Some("npm test".to_string()),
                dev: Some("npm run dev".to_string()),
            },
        }
    }

    pub fn python_project() -> ProjectTemplate {
        ProjectTemplate {
            name: "Python Project".to_string(),
            description: "A new Python project".to_string(),
            languages: vec!["python".to_string()],
            files: vec![],
            commands: ProjectCommands {
                build: None,
                run: Some("python main.py".to_string()),
                test: Some("pytest".to_string()),
                dev: Some("python -m pytest --watch".to_string()),
            },
        }
    }

    pub fn go_project() -> ProjectTemplate {
        ProjectTemplate {
            name: "Go Project".to_string(),
            description: "A new Go project".to_string(),
            languages: vec!["go".to_string()],
            files: vec![],
            commands: ProjectCommands {
                build: Some("go build".to_string()),
                run: Some("go run main.go".to_string()),
                test: Some("go test ./...".to_string()),
                dev: Some("go run main.go".to_string()),
            },
        }
    }

    pub fn get_template(project_type: &str) -> Option<ProjectTemplate> {
        match project_type {
            "rust" => Some(Self::rust_project()),
            "web" => Some(Self::web_project()),
            "python" => Some(Self::python_project()),
            "go" => Some(Self::go_project()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_project_builder() {
        let config = ProjectBuilder::new("test".to_string())
            .with_auto_format(true)
            .with_languages(vec!["rust".to_string()])
            .build();

        assert_eq!(config.name, "test");
        assert!(config.auto_format);
    }

    #[test]
    fn test_template_manager() {
        let template = TemplateManager::rust_project();
        assert_eq!(template.name, "Rust Project");
        assert!(template.commands.build.is_some());
    }

    #[test]
    fn test_get_template() {
        let template = TemplateManager::get_template("web");
        assert!(template.is_some());
        let t = template.unwrap();
        assert_eq!(t.name, "Web Project");
    }
}
