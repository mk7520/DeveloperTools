// src/file_handler.rs - File operations manager

use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub name: String,
    pub extension: String,
    pub size: u64,
    pub is_directory: bool,
    pub created: u64,
    pub modified: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    pub path: String,
    pub content: String,
    pub encoding: String,
    pub line_count: usize,
}

pub struct FileHandler {
    base_path: PathBuf,
}

impl FileHandler {
    pub fn new(base_path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    /// List files in directory
    pub fn list_files(&self, relative_path: &str) -> Result<Vec<FileInfo>> {
        let path = self.base_path.join(relative_path);
        
        if !path.exists() {
            return Ok(Vec::new());
        }

        let mut files = Vec::new();
        
        for entry in fs::read_dir(&path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            let path = entry.path();
            
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            let extension = path
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or("")
                .to_string();

            files.push(FileInfo {
                path: path.to_string_lossy().to_string(),
                name,
                extension,
                size: metadata.len(),
                is_directory: metadata.is_dir(),
                created: 0,
                modified: 0,
            });
        }

        Ok(files)
    }

    /// Read file content
    pub fn read_file(&self, relative_path: &str) -> Result<FileContent> {
        let path = self.base_path.join(relative_path);
        
        if !path.exists() {
            anyhow::bail!("File not found: {}", relative_path);
        }

        let content = fs::read_to_string(&path)
            .context("Failed to read file")?;

        let line_count = content.lines().count();

        Ok(FileContent {
            path: relative_path.to_string(),
            content,
            encoding: "utf-8".to_string(),
            line_count,
        })
    }

    /// Write file content
    pub fn write_file(&self, relative_path: &str, content: &str) -> Result<()> {
        let path = self.base_path.join(relative_path);
        
        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&path, content)
            .context("Failed to write file")?;

        Ok(())
    }

    /// Create new file
    pub fn create_file(&self, relative_path: &str) -> Result<()> {
        let path = self.base_path.join(relative_path);
        
        if path.exists() {
            anyhow::bail!("File already exists: {}", relative_path);
        }

        fs::File::create(&path)?;
        Ok(())
    }

    /// Delete file
    pub fn delete_file(&self, relative_path: &str) -> Result<()> {
        let path = self.base_path.join(relative_path);
        
        if !path.exists() {
            anyhow::bail!("File not found: {}", relative_path);
        }

        fs::remove_file(&path)?;
        Ok(())
    }

    /// Detect file language
    pub fn detect_language(&self, file_path: &str) -> String {
        match Path::new(file_path).extension().and_then(|e| e.to_str()) {
            Some("rs") => "rust",
            Some("js") => "javascript",
            Some("jsx") => "javascript",
            Some("ts") => "typescript",
            Some("tsx") => "typescript",
            Some("py") => "python",
            Some("go") => "go",
            Some("java") => "java",
            Some("cpp") | Some("cc") | Some("cxx") => "cpp",
            Some("c") => "c",
            Some("h") => "c",
            Some("css") => "css",
            Some("html") => "html",
            Some("json") => "json",
            Some("yaml") | Some("yml") => "yaml",
            Some("md") => "markdown",
            Some("sql") => "sql",
            _ => "text",
        }.to_string()
    }

    /// Check if file is text
    pub fn is_text_file(&self, file_path: &str) -> bool {
        let ext = Path::new(file_path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        matches!(
            ext,
            "rs" | "js" | "ts" | "py" | "go" | "java" | "c" | "cpp"
                | "css" | "html" | "json" | "md" | "sql" | "yaml" | "yml"
                | "txt" | "log" | "sh" | "bash"
        )
    }

    /// Get file size readable format
    pub fn format_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_idx = 0;

        while size >= 1024.0 && unit_idx < UNITS.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }

        format!("{:.2} {}", size, UNITS[unit_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_file_handler_create() {
        let dir = tempdir().unwrap();
        let handler = FileHandler::new(dir.path());
        
        let result = handler.create_file("test.txt");
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_handler_write_read() {
        let dir = tempdir().unwrap();
        let handler = FileHandler::new(dir.path());
        
        let content = "Hello, World!";
        let write_result = handler.write_file("test.txt", content);
        assert!(write_result.is_ok());

        let read_result = handler.read_file("test.txt");
        assert!(read_result.is_ok());
        let file_content = read_result.unwrap();
        assert_eq!(file_content.content, content);
    }

    #[test]
    fn test_detect_language() {
        let handler = FileHandler::new(".");
        
        assert_eq!(handler.detect_language("main.rs"), "rust");
        assert_eq!(handler.detect_language("app.js"), "javascript");
        assert_eq!(handler.detect_language("main.py"), "python");
    }

    #[test]
    fn test_format_size() {
        assert_eq!(FileHandler::format_size(512), "512.00 B");
        assert_eq!(FileHandler::format_size(1024), "1.00 KB");
    }
}
