use rusqlite::Connection;
use std::env;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <folder_path>", args[0]);
        std::process::exit(1);
    }

    // Extract folder path from command-line arguments
    let folder_path = &args[1];

    // Define the path for the SQLite database file
    let db_path = format!("{}/0_SQL_allfiles.db", folder_path);

    // Connect to SQLite database or create one if it doesn't exist
    let conn = Connection::open(&db_path)?;

    // Create a table to store file mappings if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS file_mappings (
            filename TEXT NOT NULL,
            filepath TEXT NOT NULL
        )",
        [],
    )?;

    // Iterate over the files in the specified folder and insert them into the database
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let filename = entry.file_name().to_string_lossy().to_string();
            let filepath = entry.path().to_str().unwrap().to_string();

            // Insert file mapping into the database
            conn.execute(
                "INSERT INTO file_mappings (filename, filepath) VALUES (?, ?)",
                &[&filename, &filepath],
            )?;
        }
    }

    println!("File mapping completed and stored in '{}'.", db_path);

    Ok(())
}



