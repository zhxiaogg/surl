#![feature(async_closure)]

mod cmds;
mod server;
mod utils;

use cmds::parse_cmd;

fn main() {
    let mut runnable = parse_cmd();
    runnable.as_mut().run().unwrap();
}
