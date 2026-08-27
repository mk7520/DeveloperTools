// src/snippet_manager.rs - Code snippet management

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub name: String,
    pub description: String,
    pub code: String,
    pub language: String,
    pub category: String,
    pub tags: Vec<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub usage_count: u32,
}

pub struct SnippetManager {
    snippets: HashMap<String, Snippet>,
}

impl SnippetManager {
    pub fn new() -> Self {
        Self {
            snippets: HashMap::new(),
        }
    }

    /// Create new snippet
    pub fn create_snippet(
        &mut self,
        name: String,
        code: String,
        language: String,
        category: String,
        description: Option<String>,
        tags: Option<Vec<String>>,
    ) -> String {
        let id = Uuid::new_v4().to_string();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let snippet = Snippet {
            id: id.clone(),
            name,
            description: description.unwrap_or_default(),
            code,
            language,
            category,
            tags: tags.unwrap_or_default(),
            created_at: now,
            updated_at: now,
            usage_count: 0,
        };

        self.snippets.insert(id.clone(), snippet);
        id
    }

    /// Get snippet by ID
    pub fn get_snippet(&self, id: &str) -> Option<&Snippet> {
        self.snippets.get(id)
    }

    /// Get snippet and increment usage
    pub fn use_snippet(&mut self, id: &str) -> Option<Snippet> {
        if let Some(snippet) = self.snippets.get_mut(id) {
            snippet.usage_count += 1;
            Some(snippet.clone())
        } else {
            None
        }
    }

    /// Find snippets by language
    pub fn find_by_language(&self, language: &str) -> Vec<&Snippet> {
        self.snippets
            .values()
            .filter(|s| s.language == language)
            .collect()
    }

    /// Find snippets by category
    pub fn find_by_category(&self, category: &str) -> Vec<&Snippet> {
        self.snippets
            .values()
            .filter(|s| s.category == category)
            .collect()
    }

    /// Find snippets by tag
    pub fn find_by_tag(&self, tag: &str) -> Vec<&Snippet> {
        self.snippets
            .values()
            .filter(|s| s.tags.contains(&tag.to_string()))
            .collect()
    }

    /// Search snippets by name or description
    pub fn search(&self, query: &str) -> Vec<&Snippet> {
        let query = query.to_lowercase();
        self.snippets
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query)
                    || s.description.to_lowercase().contains(&query)
                    || s.code.to_lowercase().contains(&query)
            })
            .collect()
    }

    /// Update snippet
    pub fn update_snippet(
        &mut self,
        id: &str,
        name: Option<String>,
        code: Option<String>,
        description: Option<String>,
        tags: Option<Vec<String>>,
    ) -> Option<()> {
        let snippet = self.snippets.get_mut(id)?;
        
        if let Some(n) = name {
            snippet.name = n;
        }
        if let Some(c) = code {
            snippet.code = c;
        }
        if let Some(d) = description {
            snippet.description = d;
        }
        if let Some(t) = tags {
            snippet.tags = t;
        }

        snippet.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        Some(())
    }

    /// Delete snippet
    pub fn delete_snippet(&mut self, id: &str) -> bool {
        self.snippets.remove(id).is_some()
    }

    /// Get all snippets
    pub fn list_snippets(&self) -> Vec<&Snippet> {
        self.snippets.values().collect()
    }

    /// Get most used snippets
    pub fn most_used(&self, limit: usize) -> Vec<&Snippet> {
        let mut snippets: Vec<&Snippet> = self.snippets.values().collect();
        snippets.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
        snippets.into_iter().take(limit).collect()
    }

    /// Get recently updated snippets
    pub fn recently_updated(&self, limit: usize) -> Vec<&Snippet> {
        let mut snippets: Vec<&Snippet> = self.snippets.values().collect();
        snippets.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
        snippets.into_iter().take(limit).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_snippet() {
        let mut manager = SnippetManager::new();
        let id = manager.create_snippet(
            "My Snippet".to_string(),
            "fn main() {}".to_string(),
            "rust".to_string(),
            "Functions".to_string(),
            Some("A simple main function".to_string()),
            Some(vec!["basic".to_string()]),
        );

        assert!(!id.is_empty());
        let snippet = manager.get_snippet(&id);
        assert!(snippet.is_some());
        assert_eq!(snippet.unwrap().name, "My Snippet");
    }

    #[test]
    fn test_find_by_language() {
        let mut manager = SnippetManager::new();
        manager.create_snippet(
            "Rust Snippet".to_string(),
            "fn main() {}".to_string(),
            "rust".to_string(),
            "Basic".to_string(),
            None,
            None,
        );

        let results = manager.find_by_language("rust");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_search() {
        let mut manager = SnippetManager::new();
        manager.create_snippet(
            "Print Function".to_string(),
            "println!(\"Hello\");".to_string(),
            "rust".to_string(),
            "IO".to_string(),
            Some("Print to console".to_string()),
            None,
        );

        let results = manager.search("print");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_usage_count() {
        let mut manager = SnippetManager::new();
        let id = manager.create_snippet(
            "Test".to_string(),
            "test".to_string(),
            "test".to_string(),
            "test".to_string(),
            None,
            None,
        );

        manager.use_snippet(&id);
        manager.use_snippet(&id);

        let snippet = manager.get_snippet(&id).unwrap();
        assert_eq!(snippet.usage_count, 2);
    }
}
