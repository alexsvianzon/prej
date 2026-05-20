// daemon

use tokio::fs;

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::net::UnixListener;
use anyhow::Error;

use shared::constants;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let path = format!("/var/run/{}.sock", constants::NAME);

    if fs::try_exists(&path).await? {
        fs::remove_file(&path).await?;
    }

    let listen = UnixListener::bind(path).unwrap();

    loop {
        match listen.accept().await {
            Ok((stream, _addr)) => {
                println!("new client!");
            }
            Err(e) => panic!("{}", e),
        }
    }

    Ok(())
}
