// socket
// 
// the function(s) should be able to
// 1. create a connection to the socket
// 2. send a request and write the content to the database
// 3. read responses and wait for the correct one

use shared::{protocol, constants};

use std::io;

use tokio::net::UnixStream;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};

use anyhow::Error;

use rusqlite::Connection;

use uuid::Uuid;

pub async fn request_and_wait(conn: &Connection, content: protocol::Content) -> Result<(), Error> {
    let mut stream = UnixStream::connect(format!("/var/run/{}.sock", constants::NAME)).await?;

    let uuid = Uuid::new_v4();
    let out_message = serde_json::to_string(&protocol::Message {
        content,
    })?;
    
    conn.execute(
        "INSERT INTO commands (uuid, request) VALUES (?1, ?2)",
        (&uuid.to_string(), &out_message),
    )?;

    let out_notification = serde_json::to_string(&protocol::Notification {
        uuid,
        kind: protocol::Kind::Request,
    })?;

    stream.write_all(format!("{}\n", out_notification).as_bytes()).await?;

    stream.readable().await?;
    let mut buf = [0; 1024];

    match stream.try_read(&mut buf) {
        Ok(_) => {
            println!("{}", String::from_utf8(buf.to_vec())?);
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
        Err(e) => { return Err(e.into()); }
    }

    Ok(())
}
