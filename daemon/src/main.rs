// daemon

use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tokio::net::UnixListener;

use shared::constants;

#[tokio::main]
async fn main() {
    let listen = UnixListener::bind(format!("/var/run/{}.sock", constants::NAME)).unwrap();
    loop {
        match listen.accept().await {
            Ok((stream, _addr)) => {
                println!("new client!");
            }
            Err(e) => panic!("{}", e),
        }
    }
}
