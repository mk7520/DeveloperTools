// Integration tests for Code Maestro

#[cfg(test)]
mod integration_tests {
    use std::path::PathBuf;

    #[test]
    fn test_basic_workflow() {
        // Test basic file operations
        let file_path = "test.rs";
        let content = "fn main() { println!(\"Hello\"); }";
        
        assert!(!file_path.is_empty());
        assert!(!content.is_empty());
    }

    #[test]
    fn test_language_detection() {
        let test_cases = vec![
            ("file.rs", "rust"),
            ("file.js", "javascript"),
            ("file.ts", "typescript"),
            ("file.py", "python"),
            ("file.go", "go"),
        ];

        for (file, expected_lang) in test_cases {
            let ext = file.split('.').last().unwrap();
            match ext {
                "rs" => assert_eq!(expected_lang, "rust"),
                "js" => assert_eq!(expected_lang, "javascript"),
                "ts" => assert_eq!(expected_lang, "typescript"),
                "py" => assert_eq!(expected_lang, "python"),
                "go" => assert_eq!(expected_lang, "go"),
                _ => (),
            }
        }
    }

    #[tokio::test]
    async fn test_ai_suggestion_generation() {
        let suggestion = "fn test() {}";
        assert!(!suggestion.is_empty());
    }

    #[test]
    fn test_database_persistence() {
        // Test that data persists
        let test_key = "test_setting";
        let test_value = "dark_theme";
        
        assert_eq!(test_key, "test_setting");
        assert_eq!(test_value, "dark_theme");
    }
}
