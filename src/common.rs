use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericBody<P: Serialize> {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: P,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericMessage<P: Serialize> {
    pub src: String,
    pub dest: String,
    pub body: GenericBody<P>,
}

pub fn send_message<P: Serialize>(message: GenericMessage<P>, stdout: &mut StdoutLock) {
    let msg = serde_json::to_string(&message).unwrap() + "\n";
    let _ = stdout.write_all(msg.as_bytes());
}
