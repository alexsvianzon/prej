// protocol

use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Request {
    Ping,
}

#[derive(Serialize, Deserialize)]
pub enum Response {
    Pong,
}

#[derive(Serialize, Deserialize)]
pub enum Content {
    Request(Request),
    Response(Response),
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub msg: Content,
}
