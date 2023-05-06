use serde::{Deserialize, Serialize};
use std::io::Write;

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    dest: String,
    body: Body,
}

fn main() {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    for input in inputs {
        if input.is_err() {
            continue;
        }

        let message = input.unwrap();

        let payload = match message.body.payload {
            Payload::Init { .. } => Payload::InitOk,
            Payload::Echo { echo } => Payload::EchoOk { echo },
            Payload::EchoOk { .. } => {
                continue;
            }
            Payload::InitOk => {
                continue;
            }
        };

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
}
