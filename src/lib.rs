// lib.rs — contains all database logic for the CLI todo app.
// This module is imported by main.rs to keep DB concerns separate from CLI concerns.
use rusqlite::{Connection, Result, params};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;


/// Represents a single todo entry, mapping directly to a row in the `todo` table.
#[derive(Debug)]
pub struct EntryTodo {
    pub id: i32,          // Auto-assigned by SQLite on insert
    pub title: String,
    pub description: String,
    pub completed: bool,  // Stored as 0/1 in SQLite, rusqlite maps it to bool
}

#[derive(Debug)]
pub struct AppState {
    pub pool: Pool<SqliteConnectionManager>,
}

impl EntryTodo {
    pub fn boo() {
        println!("An implementation for todo!");
    }

    /// Inserts this todo into the database.
    /// `id` is ignored — SQLite assigns its own via AUTOINCREMENT.
    pub fn add(&self, state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
        let conn = state.pool.get()?;
        conn.execute(
            "INSERT INTO todo (title, description, completed) VALUES (?1, ?2, ?3)",
            params![self.title, self.description, self.completed],
        )?;
        Ok(())
    }
    /// Fetches all todos from the database and returns them as a Vec<EntryTodo>.
    /// `query_map` iterates each row and maps it to an EntryTodo struct.
    /// `collect::<Result<Vec<_>>>()` folds the iterator into a single Result,
    /// returning an error if any row fails to parse.
    pub fn list(state: &AppState) -> Result<Vec<EntryTodo>, Box<dyn std::error::Error>> {
        let conn = state.pool.get()?;        
        let mut statement = conn.prepare("SELECT * FROM todo")?;
        let map_s = statement.query_map([], |row| {
            Ok(EntryTodo {
                id: row.get(0)?,
                title: row.get(1)?,
                description: row.get(2)?,
                completed: row.get(3)?
            })
        })?;
        let final_s: Vec<_> = map_s.collect::<Result<Vec<_>>>()?;
        Ok(final_s)}


    /// Drops the entire `todo` table, permanently deleting all entries.
    pub fn erase(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
        let conn = state.pool.get()?;               
        conn.execute("DROP TABLE IF EXISTS todo", [])?;
        Ok(())
    }

    /// Deletes a single todo by its ID.
    pub fn delete(state: &AppState, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let conn = state.pool.get()?;               
        conn.execute("DELETE FROM todo WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Flips the `completed` status of a todo between true and false.
    /// Uses SQLite's `NOT` operator to invert the current value in one query.
    pub fn toggle(state: &AppState, id: i32) -> Result<(), Box<dyn std::error::Error>> {
        let conn = state.pool.get()?;               
        conn.execute(
            "UPDATE todo SET completed = NOT completed WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }




}

/// Opens (or creates) the SQLite database file `todo.db` in the current directory.
/// Returns a `Connection` which is used for all subsequent DB operations.
pub fn create_connection() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;
    Ok(conn)
}

pub fn create_app_state() -> Result<AppState, r2d2::Error> {
    let manager = r2d2_sqlite::SqliteConnectionManager::file("todo.db");
    let manager_pool = Pool::builder().build(manager)?;

    Ok(AppState{pool: manager_pool})
}

/// Creates the `todo` table if it doesn't already exist.
/// Safe to call every time the app starts — `IF NOT EXISTS` prevents duplication.
pub fn verify_db(state: &AppState) -> Result<(), Box<dyn std::error::Error>> {
    let conn = state.pool.get()?;                   
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(())
}

/// Prints available CLI commands to stdout.
pub fn help() {
    let title = "Available Commands:";
    let text = "
    Usage:
    - add <title> <description>
    - list
    - delete <id>
    - toggle <id>
    - erase (erases whole database)
    ";
    println!("{}", title);
    println!("{}", text)
}