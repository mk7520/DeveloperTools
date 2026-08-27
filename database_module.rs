// Database module - Handle persistence

use crate::error::{MaestroError, Result};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub id: String,
    pub theme: String,
    pub font_size: u32,
    pub language: String,
    pub auto_save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snippet {
    pub id: String,
    pub name: String,
    pub content: String,
    pub language: String,
    pub category: String,
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn =
            Connection::open(path).map_err(|e| MaestroError::Database(e.to_string()))?;

        Self::init_schema(&conn)?;

        Ok(Self { conn })
    }

    fn init_schema(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS preferences (
                id TEXT PRIMARY KEY,
                theme TEXT NOT NULL,
                font_size INTEGER NOT NULL,
                language TEXT NOT NULL,
                auto_save BOOLEAN NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snippets (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                content TEXT NOT NULL,
                language TEXT NOT NULL,
                category TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS code_history (
                id TEXT PRIMARY KEY,
                file_path TEXT NOT NULL,
                content TEXT NOT NULL,
                language TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS ai_cache (
                id TEXT PRIMARY KEY,
                context_hash TEXT NOT NULL UNIQUE,
                suggestions TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            ",
        )
        .map_err(|e| MaestroError::Database(e.to_string()))?;

        Ok(())
    }

    pub fn save_preferences(&self, prefs: &UserPreferences) -> Result<()> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO preferences (id, theme, font_size, language, auto_save)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    &prefs.id,
                    &prefs.theme,
                    prefs.font_size,
                    &prefs.language,
                    prefs.auto_save
                ],
            )
            .map_err(|e| MaestroError::Database(e.to_string()))?;

        Ok(())
    }

    pub fn load_preferences(&self, id: &str) -> Result<Option<UserPreferences>> {
        let result = self
            .conn
            .query_row(
                "SELECT id, theme, font_size, language, auto_save FROM preferences WHERE id = ?1",
                params![id],
                |row| {
                    Ok(UserPreferences {
                        id: row.get(0)?,
                        theme: row.get(1)?,
                        font_size: row.get(2)?,
                        language: row.get(3)?,
                        auto_save: row.get(4)?,
                    })
                },
            )
            .optional()
            .map_err(|e| MaestroError::Database(e.to_string()))?;

        Ok(result)
    }

    pub fn save_snippet(&self, snippet: &Snippet) -> Result<()> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO snippets (id, name, content, language, category)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    &snippet.id,
                    &snippet.name,
                    &snippet.content,
                    &snippet.language,
                    &snippet.category
                ],
            )
            .map_err(|e| MaestroError::Database(e.to_string()))?;

        Ok(())
    }

    pub fn get_snippets(&self, language: &str) -> Result<Vec<Snippet>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, content, language, category FROM snippets WHERE language = ?1")
            .map_err(|e| MaestroError::Database(e.to_string()))?;

        let snippets = stmt
            .query_map(params![language], |row| {
                Ok(Snippet {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    content: row.get(2)?,
                    language: row.get(3)?,
                    category: row.get(4)?,
                })
            })
            .map_err(|e| MaestroError::Database(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| MaestroError::Database(e.to_string()))?;

        Ok(snippets)
    }

    pub fn save_code_history(&self, file_path: &str, content: &str, language: &str) -> Result<()> {
        let id = uuid::Uuid::new_v4().to_string();
        self.conn
            .execute(
                "INSERT INTO code_history (id, file_path, content, language) VALUES (?1, ?2, ?3, ?4)",
                params![id, file_path, content, language],
            )
            .map_err(|e| MaestroError::Database(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_database_creation() {
        let file = NamedTempFile::new().unwrap();
        let db = Database::new(file.path()).unwrap();
        assert!(db.load_preferences("test").unwrap().is_none());
    }

    #[test]
    fn test_save_and_load_preferences() {
        let file = NamedTempFile::new().unwrap();
        let db = Database::new(file.path()).unwrap();

        let prefs = UserPreferences {
            id: "test".to_string(),
            theme: "dark".to_string(),
            font_size: 14,
            language: "en".to_string(),
            auto_save: true,
        };

        db.save_preferences(&prefs).unwrap();
        let loaded = db.load_preferences("test").unwrap().unwrap();

        assert_eq!(loaded.theme, "dark");
        assert_eq!(loaded.font_size, 14);
    }
}
