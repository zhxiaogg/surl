extern crate clap;
extern crate hyper;

mod cmds;
mod server;

use cmds::Cmds;
use server::HttpServer;

fn main() {
    let cmds = Cmds::parse();
    let mut runnable = cmds.runnable_cmd();
    runnable.as_mut().run().unwrap();
}
