// daemon

use shared::constants;

use std::sync::Arc;
use std::io;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::fs;
use tokio::net::UnixListener;
use tokio::sync::{mpsc, Mutex};

use anyhow::Error;

async fn worker(id: u32, rx: Arc<Mutex<mpsc::Receiver<String>>>) {
    loop {
        let request = {
            let mut guard = rx.lock().await;
            guard.recv().await
        };

        match request {
            Some(request) => {
                println!("Worker {} processing request: '{}'", id, request);
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

    let (tx, mut rx) = mpsc::channel::<String>(32);

    let rx_arc = Arc::new(Mutex::new(rx));

    for i in 0..4 {
        let rx_clone = rx_arc.clone();
        tokio::spawn(worker(i, rx_clone));
    }

    let listen = UnixListener::bind(path).unwrap();

    loop {
        match listen.accept().await {
            Ok((stream, _addr)) => {
                let mut reader = BufReader::new(stream);
                let mut lines = reader.lines();

                while let Some(line) = lines.next_line().await? {
                    tx.send(line).await?;
                }
            }
            Err(e) => panic!("{}", e),
        }
    }
}
