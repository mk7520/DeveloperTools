// src/modules/core/config.rs - Configuration management

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub editor: EditorConfig,
    pub ai: AIConfig,
    pub language_servers: Vec<LanguageServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub theme: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
    pub tab_size: u32,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub font_family: String,
    pub font_size: u32,
    pub line_numbers: bool,
    pub mini_map: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub enabled: bool,
    pub provider: String,
    pub model: String,
    pub auto_complete: bool,
    pub suggestion_delay_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageServerConfig {
    pub language: String,
    pub command: String,
    pub args: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app: AppConfig {
                name: "Code Maestro".to_string(),
                version: "0.1.0".to_string(),
                theme: "One Dark Pro".to_string(),
                language: "en".to_string(),
            },
            editor: EditorConfig {
                tab_size: 4,
                insert_spaces: true,
                word_wrap: true,
                font_family: "Fira Code".to_string(),
                font_size: 14,
                line_numbers: true,
                mini_map: true,
            },
            ai: AIConfig {
                enabled: true,
                provider: "openai".to_string(),
                model: "gpt-4".to_string(),
                auto_complete: true,
                suggestion_delay_ms: 500,
            },
            language_servers: vec![
                LanguageServerConfig {
                    language: "rust".to_string(),
                    command: "rust-analyzer".to_string(),
                    args: vec![],
                },
                LanguageServerConfig {
                    language: "javascript".to_string(),
                    command: "node".to_string(),
                    args: vec!["node_modules/.bin/typescript-language-server".to_string()],
                },
                LanguageServerConfig {
                    language: "python".to_string(),
                    command: "pyls".to_string(),
                    args: vec![],
                },
            ],
        }
    }
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_language_server(&self, language: &str) -> Option<&LanguageServerConfig> {
        self.language_servers.iter().find(|ls| ls.language == language)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.app.name, "Code Maestro");
        assert_eq!(config.editor.tab_size, 4);
    }

    #[test]
    fn test_language_server_lookup() {
        let config = Config::default();
        let ls = config.get_language_server("rust");
        assert!(ls.is_some());
        assert_eq!(ls.unwrap().command, "rust-analyzer");
    }
}
