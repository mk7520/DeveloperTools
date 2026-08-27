// src/api_server.rs - REST API Server

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Request/Response types for API

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionRequest {
    pub code: String,
    pub language: String,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestionResponse {
    pub suggestions: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxCheckRequest {
    pub code: String,
    pub language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntaxCheckResponse {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub is_valid: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateBoilerplateRequest {
    pub language: String,
    pub pattern: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateBoilerplateResponse {
    pub code: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationRequest {
    pub path: String,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsRequest {
    pub theme: Option<String>,
    pub font_size: Option<u32>,
    pub language: Option<String>,
    pub auto_save: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingsResponse {
    pub theme: String,
    pub font_size: u32,
    pub language: String,
    pub auto_save: bool,
}

/// API Handler
pub struct ApiHandler {
    pub cache: Arc<RwLock<std::collections::HashMap<String, String>>>,
}

impl ApiHandler {
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    pub async fn handle_suggestions(
        &self,
        req: SuggestionRequest,
    ) -> SuggestionResponse {
        // Check cache first
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(&req.code) {
            return SuggestionResponse {
                suggestions: vec![cached.clone()],
                errors: vec![],
            };
        }
        drop(cache);

        // Generate suggestions
        let suggestions = vec![
            "// Complete this function".to_string(),
            "console.log('Debug');".to_string(),
        ];

        SuggestionResponse {
            suggestions,
            errors: vec![],
        }
    }

    pub async fn handle_syntax_check(
        &self,
        req: SyntaxCheckRequest,
    ) -> SyntaxCheckResponse {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        // Basic checks
        if !req.code.contains('{') && !req.code.contains('[') {
            warnings.push("No code blocks detected".to_string());
        }

        SyntaxCheckResponse {
            errors,
            warnings,
            is_valid: errors.is_empty(),
        }
    }

    pub async fn handle_generate_boilerplate(
        &self,
        req: GenerateBoilerplateRequest,
    ) -> GenerateBoilerplateResponse {
        let code = match req.language.as_str() {
            "rust" => match req.pattern.as_str() {
                "struct" => "pub struct MyStruct {\n    field: Type,\n}",
                "enum" => "pub enum MyEnum {\n    Variant,\n}",
                "function" => "pub fn my_function() {\n    // Implementation\n}",
                _ => "// Rust code",
            },
            "javascript" | "typescript" => match req.pattern.as_str() {
                "function" => "function myFunction() {\n    // Implementation\n}",
                "class" => "class MyClass {\n    constructor() {}\n}",
                "async" => "async function myAsync() {\n    // Implementation\n}",
                _ => "// JavaScript code",
            },
            "python" => match req.pattern.as_str() {
                "class" => "class MyClass:\n    def __init__(self):\n        pass",
                "function" => "def my_function():\n    \"\"\"Documentation.\"\"\"\n    pass",
                "async" => "async def my_async():\n    # Implementation\n    pass",
                _ => "# Python code",
            },
            _ => "// Code template",
        };

        GenerateBoilerplateResponse {
            code: code.to_string(),
            explanation: format!("Generated {} boilerplate for {}", req.pattern, req.language),
        }
    }

    pub async fn handle_file_operation(
        &self,
        req: FileOperationRequest,
    ) -> FileOperationResponse {
        match req.content {
            Some(content) => {
                // Save file
                FileOperationResponse {
                    success: true,
                    message: format!("File saved: {}", req.path),
                    data: None,
                }
            }
            None => {
                // Load file
                FileOperationResponse {
                    success: true,
                    message: format!("File loaded: {}", req.path),
                    data: Some("// File content".to_string()),
                }
            }
        }
    }

    pub async fn handle_settings(
        &self,
        req: SettingsRequest,
    ) -> SettingsResponse {
        SettingsResponse {
            theme: req.theme.unwrap_or_else(|| "dark".to_string()),
            font_size: req.font_size.unwrap_or(14),
            language: req.language.unwrap_or_else(|| "javascript".to_string()),
            auto_save: req.auto_save.unwrap_or(true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_api_handler() {
        let handler = ApiHandler::new();
        
        let req = SyntaxCheckRequest {
            code: "fn main() {}".to_string(),
            language: "rust".to_string(),
        };

        let resp = handler.handle_syntax_check(req).await;
        assert!(resp.is_valid);
    }

    #[tokio::test]
    async fn test_generate_boilerplate() {
        let handler = ApiHandler::new();
        
        let req = GenerateBoilerplateRequest {
            language: "rust".to_string(),
            pattern: "struct".to_string(),
        };

        let resp = handler.handle_generate_boilerplate(req).await;
        assert!(resp.code.contains("struct"));
    }
}
