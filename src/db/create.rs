use std::path::Path;

use chrono::Utc;
use rusqlite::Connection;
use failure::{Error, err_msg};

use db::SqliteBackend;

// Open impls
impl SqliteBackend {
    /// Creates a taskerizer database at the given path if it does not exist, and opens and returns
    /// an existing one if there already was one. Path must be a directory and not a file.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<SqliteBackend, Error> {
        let mut path = path.as_ref().to_path_buf();
        
        if !path.is_dir() {
            return Err(err_msg("Database directory path is not a directory, or we do not have permission to access it."))
        }

        path.push("tkzr_sqlite3.db");

        let existing_db = path.is_file();

        let conn = Connection::open(path)?;
        let db = SqliteBackend {
            connection: conn,
        };

        // create db tables and populate metadata table
        if !existing_db {
            db.create_tables()?;
        }
        Ok(db)
    }
}

// Create table impls
impl SqliteBackend {
    fn create_tables(&self) -> Result<(), Error> {
        // self.enable_foreign_keys_pragma()?;
        self.create_metadata_table()?;
        self.create_tasks_table()?;
        // self.create_current_table()?;
        // self.create_completed_table()?;
        Ok(())
    }

    fn create_metadata_table(&self) -> Result<(), Error> {
        let conn = &self.connection;

        conn.execute(
            "CREATE TABLE metadata (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                version TEXT NOT NULL,
                date_created TEXT NOT NULL
            )",
            &[]
        ).map_err(|e| format_err!("Could not create metadata table: {}", e))?;

        let date_created = Utc::now().to_rfc3339();
        let version = env!("CARGO_PKG_VERSION");
        conn.execute(
            "INSERT INTO metadata (id, version, date_created) VALUES (
                1,
                ?1,
                ?2
            )",
            &[&version, &date_created]
        ).map_err(|e| format_err!("Could not insert metadata into database: {}", e))?;

        Ok(())
    }

    fn create_tasks_table(&self) -> Result<(), Error> {
        let conn = &self.connection;

        conn.execute(
            "CREATE TABLE tasks (
                id INTEGER PRIMARY KEY,
                task TEXT NOT NULL,
                priority INTEGER NOT NULL,
                category INTEGER NOT NULL
            );",
            &[]
        ).map_err(|e| format_err!("Could not create tasks table: {}", e))?;

        Ok(())
    }
}

