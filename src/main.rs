extern crate packet_handler;

use std::env;
use packet_handler::network;

fn main() {
    let argument = env::args().nth(1);

    match argument {
        Some(arg) => network::handler::run(arg),
        None => network::interface::interfaces(),
    };
}
