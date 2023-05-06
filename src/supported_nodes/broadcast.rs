use crate::common::{send_message, GenericBody, GenericMessage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    Broadcast {
        message: u64,
    },
    BroadcastOk,
    Read,
    ReadOk {
        messages: Vec<u64>,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
}

type Body = GenericBody<Payload>;
type Message = GenericMessage<Payload>;

pub fn broadcast_node() -> Result<(), serde_json::Error> {
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

    let mut broadcast_messages = vec![];

    for input in inputs {
        let input = input?;

        let outgoing_payload = match input.body.payload.clone() {
            Payload::Init { .. } => {
                panic!("Message of type Init is noninitial!")
            }
            Payload::Echo { echo } => Payload::EchoOk { echo },

            Payload::Broadcast { message } => {
                broadcast_messages.push(message);
                Payload::BroadcastOk
            }
            Payload::Read => Payload::ReadOk {
                messages: broadcast_messages.clone(),
            },
            Payload::Topology { .. } => Payload::TopologyOk,
            Payload::EchoOk { .. }
            | Payload::InitOk
            | Payload::BroadcastOk
            | Payload::ReadOk { .. }
            | Payload::TopologyOk => {
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
