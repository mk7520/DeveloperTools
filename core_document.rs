// src/modules/core/document.rs - Document handling

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub path: String,
    pub language: String,
    pub content: String,
    pub version: u32,
    pub dirty: bool,
    pub encoding: String,
}

impl Document {
    pub fn new(id: String, path: String, language: String, content: String) -> Self {
        Self {
            id,
            path,
            language,
            content,
            version: 1,
            dirty: false,
            encoding: "utf-8".to_string(),
        }
    }

    pub fn update(&mut self, content: String) {
        self.content = content;
        self.version += 1;
        self.dirty = true;
    }

    pub fn mark_clean(&mut self) {
        self.dirty = false;
    }

    pub fn get_line_count(&self) -> usize {
        self.content.lines().count()
    }

    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        self.content.lines().nth(line_num)
    }

    pub fn get_lines(&self) -> Vec<&str> {
        self.content.lines().collect()
    }
}

pub struct DocumentManager {
    documents: HashMap<String, Document>,
}

impl DocumentManager {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, doc: Document) {
        self.documents.insert(doc.id.clone(), doc);
    }

    pub fn remove_document(&mut self, id: &str) -> Option<Document> {
        self.documents.remove(id)
    }

    pub fn get_document(&self, id: &str) -> Option<&Document> {
        self.documents.get(id)
    }

    pub fn get_mut_document(&mut self, id: &str) -> Option<&mut Document> {
        self.documents.get_mut(id)
    }

    pub fn list_documents(&self) -> Vec<&Document> {
        self.documents.values().collect()
    }

    pub fn dirty_documents(&self) -> Vec<&Document> {
        self.documents.values().filter(|d| d.dirty).collect()
    }
}
