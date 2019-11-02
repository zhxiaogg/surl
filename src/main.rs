extern crate clap;

mod cmds;

use cmds::{Cmd, Cmds};

fn main() {
    let cmds = Cmds::parse();
    match cmds.cmd {
        Cmd::Server { name, config: _ } => println!("run cmd {}!", name),
        Cmd::Unknown { mut app } => app.print_help().unwrap(),
    }
}
