// Editor module - Code editor integration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub range: Range,
    pub message: String,
    pub severity: DiagnosticSeverity,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

pub struct EditorState {
    pub current_file: Option<String>,
    pub open_files: HashMap<String, String>,
    pub diagnostics: HashMap<String, Vec<Diagnostic>>,
    pub selections: Vec<Range>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            current_file: None,
            open_files: HashMap::new(),
            diagnostics: HashMap::new(),
            selections: Vec::new(),
        }
    }

    pub fn open_file(&mut self, path: String, content: String) {
        self.open_files.insert(path.clone(), content);
        self.current_file = Some(path);
    }

    pub fn close_file(&mut self, path: &str) {
        self.open_files.remove(path);
        if self.current_file.as_ref().map(|p| p == path).unwrap_or(false) {
            self.current_file = None;
        }
    }

    pub fn get_current_content(&self) -> Option<String> {
        self.current_file
            .as_ref()
            .and_then(|path| self.open_files.get(path).cloned())
    }

    pub fn update_file(&mut self, path: String, content: String) {
        self.open_files.insert(path, content);
    }

    pub fn set_diagnostics(&mut self, path: String, diagnostics: Vec<Diagnostic>) {
        self.diagnostics.insert(path, diagnostics);
    }

    pub fn get_diagnostics(&self, path: &str) -> Option<&Vec<Diagnostic>> {
        self.diagnostics.get(path)
    }
}

pub struct EditorConfig {
    pub tab_size: u32,
    pub insert_spaces: bool,
    pub word_wrap: bool,
    pub line_numbers: bool,
    pub mini_map: bool,
    pub font_family: String,
    pub font_size: u32,
    pub theme: String,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            tab_size: 4,
            insert_spaces: true,
            word_wrap: true,
            line_numbers: true,
            mini_map: true,
            font_family: "Fira Code".to_string(),
            font_size: 14,
            theme: "One Dark Pro".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_state() {
        let mut state = EditorState::new();
        state.open_file("test.rs".to_string(), "fn main() {}".to_string());

        assert_eq!(state.current_file, Some("test.rs".to_string()));
        assert!(state.get_current_content().is_some());
    }

    #[test]
    fn test_diagnostics() {
        let mut state = EditorState::new();
        let diag = Diagnostic {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 5,
                },
            },
            message: "Error message".to_string(),
            severity: DiagnosticSeverity::Error,
            code: None,
        };

        state.set_diagnostics("test.rs".to_string(), vec![diag]);
        assert!(state.get_diagnostics("test.rs").is_some());
    }
}
