// protocol

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
enum Request {
    Ping,
}

#[derive(Serialize, Deserialize)]
enum Response {
    Pong,
}
