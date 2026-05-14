// main.rs

mod commands;
mod constants;

use clap::{Command, arg, command};
use rusqlite::{Connection, Result};

use std::fs;
use std::path::Path;

use crate::constants::VERSION;
use crate::constants::NAME;
use crate::constants::OS;

fn debug() {
    println!("DEBUG is enabled. Version is {VERSION} of {NAME} app on {OS}.");
    
    let test_dir_path = Path::new(constants::TEST_DIR);
    let canon_result = fs::canonicalize(test_dir_path);
    let test_dir_full = match canon_result {
        Ok(value) => value,
        Err(error) => {
            // use println here because the canon file path is not required
            println!("Could not canonicalize the test directory path because of error: {error:?}");
            test_dir_path.to_path_buf()
        },
    };
    let test_dir_display = test_dir_full.display();

    let mkdir_result = fs::create_dir_all(test_dir_path);
    match mkdir_result {
        Ok(_) => (),
        // use panic here because being unable to create all directories is a serious error
        Err(error) => panic!("Could not create directories to path {test_dir_display} because of error: {error:?}"), 
    }

    let test_file_path = [test_dir_display.to_string(), "test.txt".to_string()].concat();
    let write_result = fs::write(&test_file_path, "This file was generated while debugging {NAME}.");
    match write_result {
        Ok(_) => (),
        Err(error) => panic!("Could not create all directories to path {test_dir_display} because of error: {error:?}"),
    }

    println!("Finished DEBUG! Attempted write to a test file at {test_dir_display}.");
}

fn setup_database() -> Connection {
    let conn = Connection::open("projects.db").expect("Failed to open a connection to the database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            init_file_loc TEXT
        )",
        ()
    );

    conn
}

fn main() {
    if constants::DEBUG {
        debug();
    }

    let matches = command!()
        .subcommand_required(true)
        .version(VERSION)
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
        .get_matches();

    let conn = setup_database();

    match matches.subcommand() {
        Some(("add", sub_matches)) => commands::add(
            sub_matches,
            conn
        ),
        Some(("go", sub_matches)) => commands::go(
            sub_matches,
            conn
        ),
        _ => unreachable!("Command does not exist or was not provided"),
    }
}
