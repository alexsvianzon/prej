// daemon

use shared::{protocol, constants};

use std::sync::Arc;

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

                let in_notification: protocol::Notification = serde_json::from_str(&job.request)
                    .unwrap();

                let uuid = in_notification.uuid;
                let mut stmt = conn.prepare("SELECT uuid, request FROM commands WHERE uuid = ?1")
                    .unwrap();
                let raw: String = stmt.query_row(params![uuid.to_string()], |row| {
                    row.get("request")
                }).expect("couldnt get request");

                let in_message: protocol::Message = serde_json::from_str(&raw).unwrap();
                let in_content = in_message.content;
                let out_content = match in_content {
                    protocol::Content::Ping => protocol::Content::Pong,
                    _ => protocol::Content::Pong,
                };

                let out_message = serde_json::to_string(&protocol::Message {
                    content: out_content,
                }).unwrap();

                conn.execute(
                    "UPDATE commands SET response = ?1 WHERE uuid = ?2",
                    (out_message, &uuid.to_string()),
                ).expect("couldnt update response");

                let out_notification = serde_json::to_string(&protocol::Notification {
                    uuid,
                    kind: protocol::Kind::Response,
                }).unwrap();

                job.response.send(out_notification);
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
