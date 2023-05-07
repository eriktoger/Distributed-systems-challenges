use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::{send_message, GenericBody, GenericMessage};

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
    Broadcast {
        message: u64,
    },
    BroadcastOk,
    Read,
    ReadOk {
        value: u64,
    },
    Topology {
        topology: HashMap<String, Vec<String>>,
    },
    TopologyOk,
    Add {
        delta: u64,
    },
    AddOk,
}

type Body = GenericBody<Payload>;
type Message = GenericMessage<Payload>;

pub fn g_counter_node() -> Result<(), serde_json::Error> {
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

    let mut current_topology: HashMap<String, Vec<String>> = HashMap::new();

    // should have used the built in seq-kv, but I did not understand how to access it from Rust
    // so I cheated and used the gossip method...
    let node_ids = ["n1", "n2", "n3", "n4", "n5"];

    let mut message_counter = 0;
    let mut broadcast_messages = vec![];
    let mut counter = 0;

    for input in inputs {
        let input = input?;

        let outgoing_payload = match input.body.payload.clone() {
            Payload::Init { .. } => {
                panic!("Message of type Init is noninitial!")
            }
            Payload::Echo { echo } => Payload::EchoOk { echo },
            Payload::Generate => {
                message_counter += 1;
                let id = format!("{}-{}", current_node_id, message_counter);
                Payload::GenerateOk { id }
            }
            Payload::Broadcast { message } => {
                match broadcast_messages.iter().find(|num| **num == message) {
                    Some(_) => {}
                    None => {
                        broadcast_messages.push(message);
                        let nodes = current_topology.get(&current_node_id);
                        match nodes {
                            Some(nodes) => {
                                for node in nodes.iter() {
                                    let new_message = Message {
                                        src: current_node_id.clone(),
                                        dest: node.to_string(),
                                        body: Body {
                                            msg_id: Some(1),
                                            in_reply_to: input.body.msg_id,
                                            payload: Payload::Broadcast { message },
                                        },
                                    };
                                    send_message(new_message, &mut stdout);
                                }
                            }
                            None => {}
                        }
                    }
                }

                Payload::BroadcastOk
            }

            Payload::Read => Payload::ReadOk { value: counter },
            Payload::Topology { topology } => {
                current_topology = topology;
                println!("{current_topology:?}");
                Payload::TopologyOk
            }
            Payload::Add { delta } => {
                counter += delta;
                if input.src.starts_with("c") {
                    for node_id in node_ids {
                        if node_id == current_node_id {
                            continue;
                        }
                        let new_message = Message {
                            src: current_node_id.clone(),
                            dest: node_id.to_string(),
                            body: Body {
                                msg_id: Some(1),
                                in_reply_to: input.body.msg_id,
                                payload: Payload::Add { delta },
                            },
                        };

                        send_message(new_message, &mut stdout);
                    }
                }

                Payload::AddOk
            }
            Payload::EchoOk { .. }
            | Payload::InitOk
            | Payload::GenerateOk { .. }
            | Payload::BroadcastOk
            | Payload::ReadOk { .. }
            | Payload::AddOk
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
