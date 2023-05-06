use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
    Generate,
    GenerateOk {
        id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

fn send_message(message: Message, payload: Payload, stdout: &mut StdoutLock) {
    let new_body = Body {
        msg_id: Some(1),
        in_reply_to: message.body.msg_id,
        payload,
    };

    let response = Message {
        src: message.dest,
        dest: message.src,
        body: new_body,
    };
    let msg = serde_json::to_string(&response).unwrap() + "\n";

    let _ = stdout.write_all(msg.as_bytes());
}

fn main() -> Result<(), serde_json::Error> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let head = match inputs.next() {
        Some(head) => head?,
        None => return Ok(()),
    };

    let current_node_id = match head.body.payload.clone() {
        Payload::Init { node_id, .. } => {
            send_message(head, Payload::InitOk, &mut stdout);
            node_id
        }
        _ => panic!("First message is not of type Init!"),
    };

    let mut message_counter = 0;

    for input in inputs {
        let message = input?;

        let outgoing_payload = match message.body.payload.clone() {
            Payload::Init { .. } => {
                panic!("Message of type Init is noninitial!")
            }
            Payload::Echo { echo } => Payload::EchoOk { echo },
            Payload::Generate => {
                message_counter += 1;
                let id = format!("{}-{}", current_node_id, message_counter);
                Payload::GenerateOk { id }
            }
            Payload::EchoOk { .. } | Payload::InitOk | Payload::GenerateOk { .. } => {
                continue;
            }
        };

        send_message(message, outgoing_payload, &mut stdout);
    }
    Ok(())
}
