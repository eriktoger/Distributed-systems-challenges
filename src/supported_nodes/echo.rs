use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

use crate::common::{GenericBody, GenericMessage};

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
}

type Body = GenericBody<Payload>;
type Message = GenericMessage<Payload>;

fn send_message(message: Message, stdout: &mut StdoutLock) {
    let msg = serde_json::to_string(&message).unwrap() + "\n";
    let _ = stdout.write_all(msg.as_bytes());
}

pub fn echo_node() -> Result<(), serde_json::Error> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let mut inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let head = match inputs.next() {
        Some(head) => head?,
        None => return Ok(()),
    };

    let current_node_id = match head.body.payload.clone() {
        Payload::Init { node_id, .. } => {
            let head_reply = Message {
                src: node_id.clone(),
                dest: head.src,
                body: Body {
                    msg_id: Some(1),
                    in_reply_to: head.body.msg_id,
                    payload: Payload::InitOk,
                },
            };
            send_message(head_reply, &mut stdout);
            node_id
        }
        _ => panic!("First message is not of type Init!"),
    };

    for input in inputs {
        let input = input?;

        let outgoing_payload = match input.body.payload.clone() {
            Payload::Init { .. } => {
                panic!("Message of type Init is noninitial!")
            }
            Payload::Echo { echo } => Payload::EchoOk { echo },

            Payload::EchoOk { .. } | Payload::InitOk => {
                continue;
            }
        };

        let new_message = Message {
            src: current_node_id.clone(),
            dest: input.src,
            body: Body {
                msg_id: Some(1),
                in_reply_to: input.body.msg_id,
                payload: outgoing_payload.clone(),
            },
        };

        send_message(new_message, &mut stdout);
    }
    Ok(())
}
