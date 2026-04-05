// main.rs — entry point for the CLI todo app.
// Reads command line arguments and calls the appropriate functions from lib.rs.

use cli_todo::{EntryTodo, create_connection, delete, erase, help, list, toggle, verify_db};
use std::env::args;
use comfy_table::Table;

fn main() {
    // Collect all CLI arguments into a Vec<String>
    // args[0] is always the program name, so real args start at args[1]
    let args: Vec<String> = args().collect();

    // If no arguments were passed, print help and exit
    if args.len() == 1 {
        println!("Please pass arguments in...");
        help();
        std::process::exit(1)
    }

    // Open the database and ensure the table exists before doing anything
    let conn = create_connection().expect("Failed to connect to the database");
    verify_db(&conn).expect("Failed to verify the database");

    // Route to the correct function based on the first argument
    match args[1].as_str() {
        // Build an EntryTodo from args and insert it into the DB
        "add" => {
            let todo = EntryTodo {
                id: 0, // ignored — SQLite assigns the real id
                title: args[2].clone(),
                description: args[3].clone(),
                completed: false,
            };
            todo.add(&conn).unwrap();
        },

        // Fetch all todos and render them as a formatted table
        "list" => {
            let listing = list(&conn).unwrap();
            let mut table = Table::new();
            table.set_header(vec!["ID", "Title", "Description", "Completed"]);

            for todo in listing {
                table.add_row(vec![
                    todo.id.to_string(),
                    todo.title,
                    todo.description,
                    // Display a checkmark or blank based on completed status
                    if todo.completed { "✓".to_string() } else { " ".to_string() },
                ]);
            }
            println!("{table}");
        },

        // Parse the id argument and delete the matching row
        "delete" => {
            let id = args[2].parse::<i32>().unwrap();
            delete(&conn, id).unwrap();
        },

        // Parse the id argument and flip the completed status
        "toggle" => {
            let id = args[2].parse::<i32>().unwrap();
            toggle(&conn, id).unwrap();
        },

        // Drop the entire table — permanently deletes all todos
        "erase" => {
            erase(&conn).unwrap();
        },

        "help" | "--help" | "-h" => { help(); },

        _ => { println!("Unknown command. Type help to get information"); }
    }
}
