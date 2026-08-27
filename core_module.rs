// Core module - Contains core functionality

pub mod document;
pub mod syntax;
pub mod config;

pub use document::Document;
pub use syntax::SyntaxAnalyzer;
pub use config::Config;

/// Represents a code file in the editor
#[derive(Debug, Clone)]
pub struct CodeFile {
    pub path: String,
    pub language: String,
    pub content: String,
    pub modified: bool,
}

impl CodeFile {
    pub fn new(path: String, language: String, content: String) -> Self {
        Self {
            path,
            language,
            content,
            modified: false,
        }
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.modified = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_file_creation() {
        let file = CodeFile::new(
            "main.rs".to_string(),
            "rust".to_string(),
            "fn main() {}".to_string(),
        );
        assert_eq!(file.language, "rust");
        assert!(!file.modified);
    }
}
