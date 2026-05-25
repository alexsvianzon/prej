// protocol

use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Kind {
    Request,
    Response,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub uuid: Uuid,
    pub kind: Kind,
}

#[derive(Serialize, Deserialize)]
pub enum Content {
    Ping,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Pong,
}
