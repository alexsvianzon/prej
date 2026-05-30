// socket
// 
// the function(s) should be able to
// 1. create a connection to the socket
// 2. send a request and write the content to the database
// 3. read responses and wait for the correct one

use shared::{protocol, constants};

use std::io;

use tokio::net::UnixStream;
use tokio::io::AsyncWriteExt;

use anyhow::Error;

use rusqlite::Connection;

use uuid::Uuid;

pub async fn request_and_wait(conn: &Connection) -> Result<(), Error> {
    let mut stream = UnixStream::connect(format!("/var/run/{}.sock", constants::NAME)).await?;

    let uuid = Uuid::new_v4();
    let content = protocol::Content::Ping;
    let content_string = "ping";

    conn.execute(
        "INSERT INTO commands (uuid, content) VALUES (?1, ?2)",
        (&uuid.to_string(), &content_string),
    )?; // return result in function def

    let message_struct = protocol::Message {
        uuid: uuid,
        kind: protocol::Kind::Request,
    };

    let message = serde_json::to_string(&message_struct).unwrap();

    stream.write_all(format!("{}\n", message).as_bytes()).await?;

    stream.readable().await?;
    let mut buf = [0; 1028];

    match stream.try_read(&mut buf) {
        Ok(_) => {
            println!("{}", String::from_utf8(buf.to_vec())?);
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => (),
        Err(e) => {
            return Err(e.into());
        }
    }

    Ok(())
}
