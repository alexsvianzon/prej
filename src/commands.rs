// commands.rs

use crate::constants;

use std::fs;

use rusqlite::{Connection, params};
use ignore::Walk;
use anyhow::Error;

pub struct Project {
    id: i32,
    name: String,
    path: String,
    init_file_loc: String,
}

pub fn add(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requires a name for 'add'")
        .to_string();

    println!("used '{} add', adding project {}", constants::NAME, proj_name);
 
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
        fs::write(format!("./{}", constants::INIT_FILE_NAME), constants::INIT_FILE_CONTENT)?;
    }
    
    let project = Project { id: 0,
        name: proj_name,
        path: fs::canonicalize("./")?.to_string_lossy().to_string(),
        init_file_loc,
    };

    conn.execute(
        "INSERT INTO projects (name, path, init_file_loc) VALUES (?1, ?2, ?3)",
        (&project.name, &project.path, &project.init_file_loc),
    )?;

    Ok(())
}

pub fn list(conn: Connection) -> Result<(), Error> {
    println!("use '{} list', listing all projects: \n", constants::NAME);

    let mut statement = conn.prepare("SELECT id, name, path FROM projects")?;
    let query = statement.query_map([], |row| {
        Ok(Project {
            id: row.get("id")?,
            name: row.get("name")?,
            path: row.get("path")?,
            init_file_loc: String::new(),
        })
    })?;

    for proj in query {
        let project = proj.unwrap();

        println!("{}. Project '{}' at '{}'", project.id, project.name, project.path);
    }

    Ok(())
}

pub fn go(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requries a name for 'go'")
        .to_string();

    println!("used '{} go', going to project {}", constants::NAME, proj_name);

    // the 'go' command needs to look up the project in the database, jump to that directory, start
    // the daemon if necessary, connect to the daemon, and tell the daemon to start the processes
    // in the initfile's 'start' service
    //
    // so far, the 'go' command can look up the project in the database
    
    let mut statement = conn.prepare("SELECT id, name, path, init_file_loc FROM projects WHERE name = ?1")?;
    let query = statement.query_row(params![proj_name], |row| {
        Ok(Project {
            id: row.get("id")?,
            name: row.get("name")?,
            path: row.get("path")?,
            init_file_loc: row.get("init_file_loc")?,
        })
    });

    let project = match query {
        Ok(proj) => proj,
        Err(error) => panic!("Could not find that project in the database: {error}"),
    };

    println!("Found project '{}' at path {}.", project.name, project.path);

    Ok(())
}
