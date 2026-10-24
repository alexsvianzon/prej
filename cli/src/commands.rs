use shared::constants;
use shared::task::Task;

use crate::grapher::{ build_graph, topological_order};

use std::fs;
use std::io;
use std::collections::HashMap;

use rusqlite::{ Connection, params };
use ignore::Walk;
use anyhow::Error;
use serde::{Serialize, Deserialize };

#[derive(Serialize, Deserialize)]
struct Project {
    id: i32,
    name: String,
    path: String,
    prejfile_loc: String,
}

#[derive(Deserialize, Serialize)]
struct Cache {
    project_active: bool,
    project: Project,
}

/*

reserve this for future implementation with variables

#[derive(Deserialize)]
struct Namespace {
    tasks: HashMap<String, Task>,
}

*/

fn locate_prejfile() -> Result<String, Error> {
    let mut prejfile_loc: String = format!("./{}", constants::PREJFILE);
    let mut found: bool = false;
    for result in Walk::new("./") {
        let entry = match result {
            Ok(entry) => entry,
            Err(error) => panic!("Got an error while looking for Prejfile: {error}"),
        };

        let name = entry.file_name();
        let file_path = name.to_string_lossy();

        if file_path.ends_with(constants::PREJFILE) {
            println!("Found the Prejfile!");
            found = true;
            prejfile_loc = ["./".to_string(), file_path.to_string()].concat();

            break
        }
    }

    if !found {
        println!("Prejfile not found, creating one instead");
        fs::write(format!("./{}", constants::PREJFILE), constants::PREJFILE_CONTENT)?;
    }

    Ok(prejfile_loc)
}

fn read_prejfile(project: &Project, conn: &Connection) -> Result<Vec<u8>, Error> {
    match fs::read(&project.prejfile_loc) {
        Ok(bytes) => Ok(bytes),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            let prejfile_loc: String = locate_prejfile()?;

            conn.execute(
                "UPDATE projects SET prejfile_loc = ?1 WHERE name = ?2",
                (&prejfile_loc, &project.name),
            )?;

            Ok(fs::read(prejfile_loc)?)
        }
        Err(e) => panic!("{e}"),
    }
}

fn run_namespace(prejfile: HashMap<String, HashMap<String, Task>>, target: &str) {
    let Some(tasks) = prejfile.get(target) else {
        return;
    };

    let (id_list, task_list): (Vec<String>, Vec<&Task>) = tasks
        .iter()
        .map(|(id, task)| (id.clone(), task))
        .unzip();

    let graph = build_graph(&task_list, &id_list).unwrap();
    let order = topological_order(&graph).unwrap();

    for task_id in order {
        println!("Running task: {}", task_id);

        let task = tasks
            .get(&task_id)
            .expect("task ID in graph should exist in HashMap");

        task.run_task();
    }
}

fn get_cache() -> Result<Cache, Error> {
    let cache_bytes = fs::read(format!("{}/cache.json", constants::APPDATA_DIR))?;

    let cache: Cache = serde_json::from_str(&String::from_utf8(cache_bytes)?)?;

    Ok(cache)
}

pub fn add(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requires a name for 'add'")
        .to_string();
 
    let prejfile_loc: String = locate_prejfile()?;
    
    let project = Project { 
        id: 0,
        name: proj_name.clone(),
        path: fs::canonicalize("./")?.to_string_lossy().to_string(),
        prejfile_loc,
    };

    let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM projects WHERE name = ?1)")?;
    let project_exists: bool = stmt.query_row(params![proj_name.to_string()], |row| {
        row.get(0)
    }).expect("unable to query database");

    if project_exists == false {
        conn.execute(
            "INSERT INTO projects (name, path, prejfile_loc) VALUES (?1, ?2, ?3)",
            (&project.name, &project.path, &project.prejfile_loc),
        )?;
    } else {
        eprintln!("A project under this name already exists.");
        eprintln!("Would you like to change its path instead?");

        // TODO: implement path changing
    }

    Ok(())
}

pub fn rm(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error>{
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requires a name for 'rm'")
        .to_string();

    conn.execute("DELETE FROM projects WHERE name = ?1",
        (proj_name,),
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
            prejfile_loc: String::new(),
        })
    })?;

    for proj in query {
        let project = proj?;

        println!("{}. Project '{}' at '{}'", project.id, project.name, project.path);
    }

    Ok(())
}

pub fn go(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requires a name for 'go'")
        .to_string();

    /*
    the 'go' command should be able to loop up a project in the database, jump to that directory,
    start the daemon if necessary, connect to the daemon, and tell the daemon to start the
    processes in the init file's 'start' namespace

    the go command can switch to the directory and run tasks, unless the requested project is
    active already
     */

    let cache = get_cache()?;
    if cache.project.name == proj_name { return Ok(()) }

    let mut statement = conn.prepare("SELECT id, name, path, prejfile_loc FROM projects WHERE name = ?1")?;
    let query = statement.query_row(params![proj_name], |row| {
        Ok(Project {
            id: row.get("id")?,
            name: row.get("name")?,
            path: row.get("path")?,
            prejfile_loc: row.get("prejfile_loc")?,
        })
    });

    let project = match query {
        Ok(proj) => proj,
        Err(error) => panic!("Could not find that project in the database: {error}"),
    };

    let prejfile_bytes = read_prejfile(&project, &conn)?;

    let prejfile_str = String::from_utf8(prejfile_bytes)?;
    let prejfile: HashMap<String, HashMap<String, Task>> = yaml_serde::from_str(&prejfile_str)?;

    run_namespace(prejfile, "setup");

    let cache = Cache {
        project_active: true,
        project,
    };

    fs::write(format!("{}/cache.json", constants::APPDATA_DIR), serde_json::to_string(&cache)?)?;

    Ok(())
}

pub fn dir(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    let proj_name = matches
        .get_one::<String>("NAME")
        .expect("Requires a name for 'dir'")
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

pub fn active() -> Result<(), Error> {
    let cache = get_cache()?;

    if cache.project_active == true {
        println!("{}", cache.project.name);
    } else {
        eprintln!("There is no active project");
    }

    Ok(())
}

pub fn close(conn: Connection) -> Result<(), Error> {
    let cache = get_cache()?;

    if cache.project_active == false {
        eprintln!("There is no active project to close");
        panic!();
    }

    let project = cache.project;

    let prejfile_bytes = read_prejfile(&project, &conn)?;

    let prejfile_str = String::from_utf8(prejfile_bytes)?;
    let prejfile: HashMap<String, HashMap<String, Task>> = yaml_serde::from_str(&prejfile_str)?;

    run_namespace(prejfile, "close");

    let cache = Cache {
        project_active: true,
        project,
    };

    fs::write(format!("{}/cache.json", constants::APPDATA_DIR), serde_json::to_string(&cache)?)?;

    Ok(())
}

pub fn run(matches: &clap::ArgMatches, conn: Connection) -> Result<(), Error> {
    let target = matches
        .get_one::<String>("TARGET")
        .expect("requires a target to run")
        .to_string();

    let namespace = matches
        .get_one::<String>("namespace")
        .expect("Requires a namespace for run")
        .to_string();

    println!("Running in namespace '{}'", namespace);


    let cache = get_cache()?;

    if cache.project_active == false {
        eprintln!("there is no active project to close");
        panic!();
    }

    let project = cache.project;

    let prejfile_bytes = read_prejfile(&project, &conn)?;

    let prejfile_str = String::from_utf8(prejfile_bytes)?;
    let prejfile: HashMap<String, HashMap<String, Task>> = yaml_serde::from_str(&prejfile_str)?;

    if let Some(tasks) = prejfile.get(namespace.as_str()) {
        if tasks.contains_key(&target) {
            let mut required = std::collections::HashSet::new();
            let mut pending = vec![target.clone()];
            while let Some(task_id) = pending.pop() {
                if required.insert(task_id.clone()) {
                    if let Some(task) = tasks.get(&task_id) {
                        pending.extend(task.depends.iter().flatten().cloned());
                    }
                }
            }

            let (id_list, task_list): (Vec<String>, Vec<&Task>) = tasks
                .iter()
                .filter(|(id, _)| required.contains(*id))
                .map(|(id, task)| (id.clone(), task))
                .unzip();

            let graph = build_graph(&task_list, &id_list).unwrap();
            let order = topological_order(&graph).unwrap();

            for name in order.into_iter().filter(|name| required.contains(name)) {
                let task = tasks
                    .get(&name)
                    .expect("task ID in graph should exist in HashMap");
                println!("executing task: {}", name);

                task.run_task();
            }
        }
    } else {
        eprintln!("Could not find namespace '{}'", namespace);
    }

    let cache = Cache {
        project_active: true,
        project,
    };

    fs::write(format!("{}/cache.json", constants::APPDATA_DIR), serde_json::to_string(&cache)?)?;

    Ok(())
}
