// commands.rs

use crate::constants;
use crate::constants::NAME;

use std::fs;

use rusqlite::Connection;
use ignore::Walk;
use anyhow::Error;

pub struct Project {
    id: i32,
    name: String,
    path: String,
    init_file_loc: String,
}

pub fn add(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    println!("used '{NAME} add', adding project {}",
        matches
            .get_one::<String>("NAME")
            .expect("Requires a name for 'add'"));
 
    let mut init_file_loc: String = format!("./{}", constants::INIT_FILE_NAME);
    let mut found: bool = false;
    for result in Walk::new("./") {
        let entry = match result {
            Ok(entry) => entry,
            Err(error) => panic!("Got an error while looking for init file: {error}"),
        };

        let name = entry.file_name();
        let file_path = name.to_string_lossy();

        if file_path.ends_with(constants::INIT_FILE_NAME) {
            println!("Found the init file!");
            found = true;
            init_file_loc = ["./".to_string(), file_path.to_string()].concat();

            break
        }
    }

    if !found {
        println!("Init file not found, creating one instead");
        let write_res = fs::write(format!("./{}", constants::INIT_FILE_NAME), constants::INIT_FILE_CONTENT)?;
    }
    
    let mut project = Project {
        id: 0,
        name: matches
            .get_one::<String>("NAME")
            .expect("Requires a name for 'add'")
            .to_string(),
        path: fs::canonicalize("./")?.to_string_lossy().to_string(),
        init_file_loc,
    };

    conn.execute(
        "INSERT INTO projects (name, path, init_file_loc) VALUES (?1, ?2, ?3)",
        (&project.name, &project.path, &project.init_file_loc),
    )?;

    Ok(())
}

pub fn go(matches: &clap::ArgMatches, conn: Connection) {
    println!("used '{NAME} go', going to project {}",
        matches
            .get_one::<String>("NAME")
            .expect("Requires a name for 'go'"));
}
