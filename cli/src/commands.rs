// commands.rs

use shared::constants;

use std::fs;
use std::io;

use rusqlite::{Connection, params};

use ignore::Walk;

use anyhow::Error;

use rust_yaml::Yaml;
use rust_yaml::Value;

pub struct Project {
    id: i32,
    name: String,
    path: String,
    init_file_loc: String,
}

#[derive(PartialEq)]
enum TaskFrom {
    All,
    Setup,
    Demand,
    Close,
    Input,
}

#[derive(Debug)]
struct Task {
    name: String,
    cmd: String,
    args: Vec<String>,
    depends: Vec<String>,
}

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

fn parse_task(task_name: String, task_content: rust_yaml::Value) -> Result<Task, Error> {
    let mut task_command: String = String::new();
    let mut task_args: Vec<String> = Vec::new();
    let mut task_deps: Vec<String> = Vec::new();

    match task_content {
        rust_yaml::Value::Mapping(map) => {
            for (key, value) in map {
                match key.as_str() {
                    Some("cmd") => task_command = value.to_string(),
                    Some("args") => {
                        match value {
                            rust_yaml::Value::Sequence(args) => {
                                for arg in args {
                                    task_args.push(arg.to_string());
                                }
                            }

                            _ => panic!("no args"),
                        }
                    }

                    _ => println!("unknown arg {}", key),
                }
            }
        }

        _ => panic!("AHH"),
    }

    let task = Task {
        name: task_name,
        cmd: task_command,
        args: task_args,
        depends: task_deps,
    };

    Ok(task)
}

fn parse_namespace(namespace: rust_yaml::Value, include_namespace: bool) -> Result<Vec<Task>, Error> {
    let mut out = Vec::new();

    if include_namespace {
        match namespace {
            rust_yaml::Value::Mapping(map) => {
                for (task_name, task_content) in map {
                    out.push(parse_task(task_name.to_string(), task_content)?);
                }
            }
            _ => panic!("AHH"),
        }
    }

    Ok(out)
}

fn parse_yaml(yaml: rust_yaml::Value, from: TaskFrom) -> Result<Vec<Task>, Error> {
    let mut out = Vec::new();

    match yaml {
        rust_yaml::Value::Mapping(map) => {
            for (key, value) in map {
                match key.as_str() {
                    Some("version") => println!("the 'version' value is for debugging only!"),
                    Some("setup") => {
                        out.append(&mut parse_namespace(
                            value,
                            from == TaskFrom::Setup || from == TaskFrom::All,
                        )?);
                    }
                    Some("demand") => {
                        out.append(&mut parse_namespace(
                            value,
                            from == TaskFrom::Demand || from == TaskFrom::All,
                        )?);
                    }
                    Some("close") => {
                        out.append(&mut parse_namespace(
                            value,
                            from == TaskFrom::Close || from == TaskFrom::All,
                        )?);
                    }
                    Some("input") => {
                        out.append(&mut parse_namespace(
                            value,
                            from == TaskFrom::Input || from == TaskFrom::All,
                        )?);
                    }
                    _ => (),
                }
            }
        }
        _ => panic!("AHH"),
    }

    Ok(out)
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

    let yaml = Yaml::new();

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

    let prejfile = String::from_utf8(prejfile_bytes).unwrap();
    let prejfile_yaml: rust_yaml::Value = yaml.load_str(&prejfile)?;

    let tasks = parse_yaml(prejfile_yaml, TaskFrom::All)?;

    for task in tasks {
        println!("{:#?}", task);
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
