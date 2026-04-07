use cli_todo::*;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;


#[cfg(test)]
    /// Creates a temporary in-memory SQLite DB for testing.
    /// Nothing is written to disk — the DB is destroyed when the test ends.
    fn get_test_state() -> AppState {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::builder().build(manager).unwrap();
        let state = AppState { pool };
        verify_db(&state).unwrap();
        state
    }

    /// Verifies that adding a todo results in exactly one row in the DB.
    #[test]
    fn test_add_todo() {
        let _conn = get_test_state();
        let _test_entry = EntryTodo {
            id: 123, // ignored by SQLite
            title: "testing title".to_string(),
            description: "test of test".to_string(),
            completed: true,
        };
        _test_entry.add(&_conn).unwrap();

        let _test_list = EntryTodo::list(&_conn).unwrap();
        assert_eq!(_test_list.len(), 1);
    }

    /// Verifies that deleting a todo by its DB-assigned id removes it from the list.
    #[test]
    fn test_delete_todo() {
        let _conn = get_test_state();
        let _test_entry = EntryTodo {
            id: 123,
            title: "testing title".to_string(),
            description: "test of test".to_string(),
            completed: true,
        };
        _test_entry.add(&_conn).unwrap();

        // last_insert_rowid() gets the real id assigned by SQLite
        let delete_id= _conn.pool.get().unwrap().last_insert_rowid() as i32;
        let _delete = EntryTodo::delete(&_conn, delete_id);

        let _test_list = EntryTodo::list(&_conn).unwrap();
        assert_eq!(_test_list.len(), 0);
    }

    /// Verifies that toggling a todo flips its completed status.
    /// Starts as false, after one toggle should be true.
    #[test]
    fn test_toggle() {
        let _conn = get_test_state();
        let _test_entry = EntryTodo {
            id: 123,
            title: "testing title".to_string(),
            description: "test of test".to_string(),
            completed: false,
        };
        _test_entry.add(&_conn).unwrap();

        let toggle_id = _conn.pool.get().unwrap().last_insert_rowid() as i32;
        EntryTodo::toggle(&_conn, toggle_id).unwrap();

        // Fetch from DB after toggle — the struct in memory is not updated
        let todos = EntryTodo::list(&_conn).unwrap();
        assert_eq!(todos[0].completed, true);
    }
