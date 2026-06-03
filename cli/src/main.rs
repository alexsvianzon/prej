// main.rs

mod commands;
mod socket;

use std::process::Command as ShellCommand;
use std::fs::File;

use shared::{protocol, constants};

use clap::{Command, arg, command};

use rusqlite::{Connection, Result};

use anyhow::Error;

fn setup_database() -> Result<Connection, Error> {
    let conn = Connection::open(format!("{}projects.db", constants::APPDATA_DIR))
        .expect("Failed to open a connection to the database");

    conn.pragma_update_and_check(None, "journal_mode", &"WAL", |_| Ok(()))?;
 
    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            init_file_loc TEXT
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
        .about("A simple registry-based project manager")
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
        _ => unreachable!("Command does not exist or was not provided"),
    }

    Ok(())
}
