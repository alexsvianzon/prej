// main.rs

mod commands;
mod socket;
mod grapher;

use std::fs;
use shared::constants;
use clap::{ Command, arg, command };
use rusqlite::{ Connection, Result };
use anyhow::Error;

fn setup_database() -> Result<Connection, Error> {
    fs::create_dir_all(constants::APPDATA_DIR)?;

    let conn = Connection::open(format!("{}/projects.db", constants::APPDATA_DIR))
        .expect("Failed to open a connection to the database");

    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))?;
 
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            name TEXT PRIMARY KEY,
            path TEXT NOT NULL,
            prejfile_loc TEXT
        )",
        ()
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS commands (
            uuid TEXT PRIMARY KEY,
            request TEXT NOT NULL,
            response TEXT,
            consumed BOOLEAN DEFAULT FALSE
        )",
        ()
    )?;

    Ok(conn)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn = setup_database()?;

    let matches = command!()
        .subcommand_required(true)
        .version(constants::VERSION)
        .about("A registry-based project manager")
        .subcommand(
            Command::new("add")
                .about("Registers a new project")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("go")
                .about("Switch to and initialize a project")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("list")
                .about("List all registered projects")
        )
        .subcommand(
            Command::new("rm")
                .about("Remove a registered project")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("dir")
                .about("Print the directory of a project")
                .arg(arg!([NAME])),
        )
        .subcommand(
            Command::new("active")
                .about("Print the active project")
        )
        .subcommand(
            Command::new("close")
                .about("Close the active project")
        )
        .subcommand(
            Command::new("run")
                .about("Run a task in the active project, and its dependencies")
                .arg(
                    arg!(-n --namespace <NAMESPACE> "The namespace the task is in")
                        .default_value("demand")
                )
                .arg(arg!([TARGET])),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => commands::add(
            sub_matches,
            conn
        )?,
        Some(("go", sub_matches)) => commands::go(
            sub_matches,
            conn
        )?,
        Some(("list", _sub_matches)) => commands::list(
            conn
        )?,
        Some(("rm", sub_matches)) => commands::rm(
            sub_matches,
            conn
        )?,
        Some(("dir", sub_matches)) => commands::dir(
            sub_matches,
            conn
        )?,
        Some(("active", _sub_matches)) => commands::active()?,
        Some(("close", _sub_matches)) => commands::close(
            conn,
        )?,
        Some(("run", sub_matches)) => commands::run(
            sub_matches,
            conn
        )?,
        _ => unreachable!("Command does not exist or was not provided"),
    }

    Ok(())
}
