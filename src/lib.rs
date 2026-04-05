// lib.rs — contains all database logic for the CLI todo app.
// This module is imported by main.rs to keep DB concerns separate from CLI concerns.

use rusqlite::{Connection, Result, params};

/// Represents a single todo entry, mapping directly to a row in the `todo` table.
#[derive(Debug)]
pub struct EntryTodo {
    pub id: i32,          // Auto-assigned by SQLite on insert
    pub title: String,
    pub description: String,
    pub completed: bool,  // Stored as 0/1 in SQLite, rusqlite maps it to bool
}

impl EntryTodo {
    pub fn boo() {
        println!("An implementation for todo!");
    }

    /// Inserts this todo into the database.
    /// `id` is ignored — SQLite assigns its own via AUTOINCREMENT.
    pub fn add(&self, conn: &Connection) -> Result<()> {
        conn.execute(
            "INSERT INTO todo (title, description, completed) VALUES (?1, ?2, ?3)",
            params![self.title, self.description, self.completed],
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

/// Creates the `todo` table if it doesn't already exist.
/// Safe to call every time the app starts — `IF NOT EXISTS` prevents duplication.
pub fn verify_db(conn: &Connection) -> Result<()> {
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

/// Fetches all todos from the database and returns them as a Vec<EntryTodo>.
/// `query_map` iterates each row and maps it to an EntryTodo struct.
/// `collect::<Result<Vec<_>>>()` folds the iterator into a single Result,
/// returning an error if any row fails to parse.
pub fn list(conn: &Connection) -> Result<Vec<EntryTodo>> {
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
    Ok(final_s)
}

/// Drops the entire `todo` table, permanently deleting all entries.
pub fn erase(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS todo", [])?;
    Ok(())
}

/// Deletes a single todo by its ID.
pub fn delete(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM todo WHERE id = ?1", params![id])?;
    Ok(())
}

/// Flips the `completed` status of a todo between true and false.
/// Uses SQLite's `NOT` operator to invert the current value in one query.
pub fn toggle(conn: &Connection, id: i32) -> Result<()> {
    conn.execute(
        "UPDATE todo SET completed = NOT completed WHERE id = ?1",
        params![id],
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


#[cfg(test)]
mod tests {
    use super::*;

    /// Creates a temporary in-memory SQLite DB for testing.
    /// Nothing is written to disk — the DB is destroyed when the test ends.
    fn get_test_conn() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        verify_db(&conn).unwrap();
        conn
    }

    /// Verifies that adding a todo results in exactly one row in the DB.
    #[test]
    fn test_add_todo() {
        let _conn = get_test_conn();
        let _test_entry = EntryTodo {
            id: 123, // ignored by SQLite
            title: "testing title".to_string(),
            description: "test of test".to_string(),
            completed: true,
        };
        _test_entry.add(&_conn).unwrap();

        let _test_list = list(&_conn).unwrap();
        assert_eq!(_test_list.len(), 1);
    }

    /// Verifies that deleting a todo by its DB-assigned id removes it from the list.
    #[test]
    fn test_delete_todo() {
        let _conn = get_test_conn();
        let _test_entry = EntryTodo {
            id: 123,
            title: "testing title".to_string(),
            description: "test of test".to_string(),
            completed: true,
        };
        _test_entry.add(&_conn).unwrap();

        // last_insert_rowid() gets the real id assigned by SQLite
        let delete_id = _conn.last_insert_rowid() as i32;
        let _delete = delete(&_conn, delete_id);

        let _test_list = list(&_conn).unwrap();
        assert_eq!(_test_list.len(), 0);
    }

    /// Verifies that toggling a todo flips its completed status.
    /// Starts as false, after one toggle should be true.
    #[test]
    fn test_toggle() {
        let _conn = get_test_conn();
        let _test_entry = EntryTodo {
            id: 123,
            title: "testing title".to_string(),
            description: "test of test".to_string(),
            completed: false,
        };
        _test_entry.add(&_conn).unwrap();

        let toggle_id = _conn.last_insert_rowid() as i32;
        toggle(&_conn, toggle_id).unwrap();

        // Fetch from DB after toggle — the struct in memory is not updated
        let todos = list(&_conn).unwrap();
        assert_eq!(todos[0].completed, true);
    }
}
