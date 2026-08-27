// src/modules/core/syntax.rs - Syntax analysis

use regex::Regex;
use std::collections::HashMap;

pub struct SyntaxAnalyzer {
    keywords: HashMap<String, Vec<String>>,
    patterns: HashMap<String, Regex>,
}

impl SyntaxAnalyzer {
    pub fn new() -> Self {
        let mut keywords = HashMap::new();

        // Rust keywords
        keywords.insert(
            "rust".to_string(),
            vec![
                "fn", "let", "mut", "const", "static", "struct", "enum", "impl", "trait",
                "pub", "priv", "use", "mod", "crate", "async", "await", "match", "if",
                "else", "for", "while", "loop", "break", "continue", "return",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        // JavaScript keywords
        keywords.insert(
            "javascript".to_string(),
            vec![
                "function", "var", "let", "const", "class", "extends", "import", "export",
                "default", "async", "await", "return", "if", "else", "for", "while",
                "switch", "case", "break", "continue", "try", "catch", "finally",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        // Python keywords
        keywords.insert(
            "python".to_string(),
            vec![
                "def", "class", "if", "elif", "else", "for", "while", "try", "except",
                "finally", "with", "import", "from", "return", "yield", "async", "await",
                "lambda", "pass", "break", "continue",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
        );

        Self {
            keywords,
            patterns: HashMap::new(),
        }
    }

    pub fn analyze(&self, code: &str, language: &str) -> SyntaxResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        match language {
            "rust" => self.analyze_rust(code, &mut errors, &mut warnings),
            "javascript" | "typescript" => self.analyze_js(code, &mut errors, &mut warnings),
            "python" => self.analyze_python(code, &mut errors, &mut warnings),
            _ => {}
        }

        SyntaxResult { errors, warnings }
    }

    fn analyze_rust(&self, code: &str, errors: &mut Vec<String>, warnings: &mut Vec<String>) {
        // Check for unclosed braces
        let open_braces = code.matches('{').count();
        let close_braces = code.matches('}').count();
        if open_braces != close_braces {
            errors.push(format!("Mismatched braces: {} opening, {} closing", open_braces, close_braces));
        }

        // Check for unclosed parentheses
        let open_parens = code.matches('(').count();
        let close_parens = code.matches(')').count();
        if open_parens != close_parens {
            errors.push(format!("Mismatched parentheses: {} opening, {} closing", open_parens, close_parens));
        }

        // Check for missing semicolons
        for line in code.lines() {
            let trimmed = line.trim();
            if trimmed.ends_with('}') || trimmed.ends_with(',') || trimmed.is_empty() {
                continue;
            }
            if !trimmed.starts_with("//") && !trimmed.ends_with(';') && !trimmed.ends_with('{') {
                if let Some(kw) = trimmed.split_whitespace().next() {
                    if !["if", "else", "for", "while", "match", "fn"].contains(&kw) {
                        warnings.push(format!("Missing semicolon: {}", trimmed));
                    }
                }
            }
        }
    }

    fn analyze_js(&self, code: &str, errors: &mut Vec<String>, warnings: &mut Vec<String>) {
        // Check for unclosed braces
        let open_braces = code.matches('{').count();
        let close_braces = code.matches('}').count();
        if open_braces != close_braces {
            errors.push(format!("Mismatched braces: {} opening, {} closing", open_braces, close_braces));
        }

        // Check for unclosed quotes
        let single_quotes = code.matches('\'').count();
        let double_quotes = code.matches('"').count();
        if single_quotes % 2 != 0 {
            warnings.push("Unclosed single quote".to_string());
        }
        if double_quotes % 2 != 0 {
            warnings.push("Unclosed double quote".to_string());
        }
    }

    fn analyze_python(&self, code: &str, errors: &mut Vec<String>, _warnings: &mut Vec<String>) {
        // Check for proper indentation
        let lines: Vec<&str> = code.lines().collect();
        let mut prev_indent = 0;

        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }

            let indent = line.len() - line.trim_start().len();
            let indent_level = indent / 4;

            if indent % 4 != 0 {
                errors.push(format!("Line {}: Inconsistent indentation", i + 1));
            }

            if indent_level > prev_indent + 1 {
                errors.push(format!("Line {}: Unexpected indent increase", i + 1));
            }

            prev_indent = indent_level;
        }
    }

    pub fn get_keywords(&self, language: &str) -> Option<&Vec<String>> {
        self.keywords.get(language)
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl SyntaxResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_issues(&self) -> bool {
        !self.errors.is_empty() || !self.warnings.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syntax_analyzer_creation() {
        let analyzer = SyntaxAnalyzer::new();
        assert!(analyzer.get_keywords("rust").is_some());
    }

    #[test]
    fn test_rust_brace_matching() {
        let analyzer = SyntaxAnalyzer::new();
        let result = analyzer.analyze("fn main() { }", "rust");
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_rust_unmatched_braces() {
        let analyzer = SyntaxAnalyzer::new();
        let result = analyzer.analyze("fn main() {", "rust");
        assert!(!result.errors.is_empty());
    }
}
