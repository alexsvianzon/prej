// protocol

use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Kind {
    Request,
    Response,
}

#[derive(Serialize, Deserialize)]
pub struct Notification {
    pub uuid: Uuid,
    pub kind: Kind,
}

#[derive(Serialize, Deserialize)]
pub enum Content {
    Ping,
    Pong,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: Content,
}
