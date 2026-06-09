// commands.rs

use shared::constants;

use std::fs;
use std::io;
use std::process::{Stdio, Command};
use std::collections::HashMap;

use rusqlite::{Connection, params};

use ignore::Walk;

use anyhow::Error;

use serde::Deserialize;

pub struct Project {
    id: i32,
    name: String,
    path: String,
    init_file_loc: String,
}

#[derive(Debug, Deserialize)]
struct Task {
    cmd: String,
    args: Option<Vec<String>>,
    depends: Option<Vec<String>>,
}

/* reserve this for future implementation with variables

#[derive(Deserialize)]
struct Namespace {
    tasks: HashMap<String, Task>,
}

*/

pub fn add(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requires a name for 'add'")
        .to_string();
 
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

pub fn rm(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error>{
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requires a name for 'rm'")
        .to_string();

    conn.execute("DELETE FROM projects WHERE name = ?1",
        ((proj_name),),
    )?;

    Ok(())
}
        
pub fn list(conn: Connection) -> Result<(), Error> {
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
    
    // the 'go' command needs to look up the project in the database, jump to that directory, start
    // the daemon if necessary, connect to the daemon, and tell the daemon to start the processes
    // in the initfile's 'start' service
    //
    // so far, the 'go' command can look up the project in the database and get the Prejfile

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

    let prejfile_bytes = match fs::read(project.init_file_loc) {
        Ok(bytes) => bytes,
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
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

            conn.execute(
                "UPDATE projects SET init_file_loc = ?1 WHERE name = ?2",
                (&init_file_loc, &project.name),
            )?;

            fs::read(init_file_loc)?
        }
        Err(e) => panic!("{e}"),
    };

    let prejfile_str = String::from_utf8(prejfile_bytes).unwrap();
    let prejfile: HashMap<String, HashMap<String, Task>> = yaml_serde::from_str(&prejfile_str)?;

    for (namespace, tasks) in prejfile {
        if namespace == "setup".to_string() {
            for (name, task) in tasks {
                println!("executing task: {}", name);

                let mut cmd = Command::new(task.cmd);

                if let Some(ref args) = task.args {
                    cmd.args(args);
                }

                cmd.stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("could not spawn task: {name}");
            }
        }
    }

    Ok(())
}

pub fn dir(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requries a name for 'dir'")
        .to_string();
    
    let mut statement = conn.prepare("SELECT path FROM projects WHERE name = ?1")?;
    let query = statement.query_row(params![proj_name], |row| {
        row.get("path")
    });

    let dir: String = match query {
        Ok(proj) => proj,
        Err(error) => panic!("Could not find that project in the database: {error}"),
    };

    println!("{}", dir.to_string());

    Ok(())
}
