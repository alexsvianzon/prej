// main.rs

mod commands;

use shared::{protocol, constants};

use std::io;

use clap::{Command, arg, command};

use rusqlite::{Connection, Result};

use tokio::io::AsyncWriteExt;
use tokio::net::UnixStream;

use anyhow::Error;

use uuid::Uuid;

fn setup_database() -> Result<Connection, Error> {
    let conn = Connection::open("projects.db").expect("Failed to open a connection to the database");

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
            content TEXT NOT NULL,
            response TEXT,
            consumed BOOL
        )",
        ()
    )?;

    Ok(conn)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let conn = setup_database()?;

    let mut stream = UnixStream::connect(format!("/var/run/{}.sock", constants::NAME)).await?;

    for _ in 0..2 {
        let uuid = Uuid::new_v4();
        let content = protocol::Content::Ping;
        let content_string = match content {
            protocol::Content::Ping => "ping",
            _ => "ping",
        };

        conn.execute(
            "INSERT INTO commands (uuid, content) VALUES (?1, ?2)",
            (&uuid.to_string(), &content_string),
        )?;

        let message_struct = protocol::Message {
            uuid: uuid,
            kind: protocol::Kind::Request,
        };

        let message = serde_json::to_string(&message_struct).unwrap();

        stream.write_all(format!("{}\n", message).as_bytes()).await?;
    }

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
        _ => unreachable!("Command does not exist or was not provided"),
    }

    Ok(())
}
