// daemon

use shared::{protocol, constants};

use std::sync::Arc;
use std::io;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs;
use tokio::net::UnixListener;
use tokio::sync::{mpsc, Mutex, oneshot};

use rusqlite::{Connection, params};

use anyhow::Error;

struct Job {
    request: String,
    response: oneshot::Sender<String>,
}

async fn worker(id: u32, rx: Arc<Mutex<mpsc::Receiver<Job>>>) {
    loop {
        let job = {
            let mut guard = rx.lock().await;
            guard.recv().await
        };

        match job {
            Some(job) => {
                let conn = Connection::open("projects.db")
                    .expect("Failed to open a connection to the database");
                
                let incoming: protocol::Message = serde_json::from_str(&job.request).unwrap();

                let mut stmt = conn.prepare("SELECT uuid, content FROM commands WHERE uuid = ?1");
                let mut statement = match stmt {
                    Ok(ok) => ok,
                    Err(error) => panic!("i give up, error: {error}"),
                };

                let content: String = statement.query_row(params![incoming.uuid.to_string()], |row| {
                    row.get("content")
                })
                .expect("could not find uuid");

                println!("{}", content);

                let response = match content.as_str() {
                    "ping" => "pong".to_string(),
                    _ => "pong".to_string(),
                };

                let response_e = match response.as_str() {
                    "pong" => protocol::Response::Pong,
                    _ => protocol::Response::Pong,
                };

                conn.execute(
                    "UPDATE commands SET response = ?1 WHERE uuid = ?2",
                    (response, incoming.uuid.to_string()),
                ).expect("message");

                let outgoing = protocol::Message {
                    uuid: incoming.uuid,
                    kind: protocol::Kind::Response,
                };

                let outgoing_ser = serde_json::to_string(&outgoing).expect("could not something");

                job.response.send(outgoing_ser);
            }
            None => break,
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let path = format!("/var/run/{}.sock", constants::NAME);

    if fs::try_exists(&path).await? {
        fs::remove_file(&path).await?;
    }

    let (tx, mut rx) = mpsc::channel::<Job>(32);

    let rx_arc = Arc::new(Mutex::new(rx));

    for i in 0..4 {
        let rx_clone = rx_arc.clone();
        tokio::spawn(worker(i, rx_clone));
    }

    let listen = UnixListener::bind(path).unwrap();

    loop {
        let (stream, _) = listen.accept().await?;

        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let (reader, mut writer) = tokio::io::split(stream);

            let reader = BufReader::new(reader);
            let mut lines = reader.lines();

            while let Some(line) = lines.next_line().await.unwrap() {
                let (os_tx, os_rx) = oneshot::channel();

                let job = Job {
                    request: line,
                    response: os_tx,
                };

                tx_clone.send(job).await;

                match os_rx.await {
                    Ok(response) => {
                        let _ = writer
                            .write_all(format!("{}\n", response).as_bytes())
                            .await;

                        println!("{}", response);
                    }

                    Err(_) => {
                        let _ = writer
                            .write_all(b"dropped\n")
                            .await;
                    }
                }
            }
        });
    }
}
