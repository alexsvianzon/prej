// protocol

use serde::{Serialize, Deserialize};

use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub enum Job {
    StartProcess,
    Response,
    GetStatus { target: Uuid },
    GetLogs { target: Uuid, messages: u8 },
    StopProcess { target: Uuid },
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub uuid: Uuid,
    pub job: Job,
}
