#![feature(async_closure)]

mod cmds;
mod server;
mod utils;

use cmds::Cmds;

fn main() {
    let mut runnable = Cmds::parse();
    runnable.as_mut().run().unwrap();
}
