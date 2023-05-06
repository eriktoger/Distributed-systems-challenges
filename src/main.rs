pub mod common;
pub mod supported_nodes;
use supported_nodes::{
    broadcast::broadcast_node, broadcast2::broadcast2_node, echo::echo_node, unique::unique_node,
};

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let node_type = &args[1];
    let _ = match node_type.as_str() {
        "echo" => echo_node(),
        "unique" => unique_node(),
        "broadcast" => broadcast_node(),
        "broadcast2" => broadcast2_node(),
        _ => {
            panic!("Use a supported node type: echo, unique, broadcast or broadcast2")
        }
    };
}
