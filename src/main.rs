#![feature(async_closure)]

mod cmds;
mod server;
mod utils;

use cmds::Cmds;

fn main() {
    let cmds = Cmds::parse();
    let mut runnable = cmds.runnable_cmd();
    runnable.as_mut().run().unwrap();
}
