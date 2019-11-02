mod startup;

use clap::App;
use clap::Arg;

use startup::StartupCmd;

use std::result::Result;
use ServerCmdName::*;

pub enum Cmd {
    Server {
        name: ServerCmdName,
        config: String,
        port: u16,
    },
    Unknown,
}

pub enum ServerCmdName {
    Start,
    Unknown,
}

impl ServerCmdName {
    fn from(name: &str) -> ServerCmdName {
        match name {
            "start" => ServerCmdName::Start,
            _ => ServerCmdName::Unknown,
        }
    }
}

pub trait RunnableCmd {
    fn run(&mut self) -> Result<(), String>;
}

struct RunnableUnknown {
    app: App<'static, 'static>,
}

impl RunnableCmd for RunnableUnknown {
    fn run(&mut self) -> Result<(), String> {
        self.app.print_help().unwrap();
        Result::Ok(())
    }
}

pub struct Cmds {
    pub cmd: Cmd,
}

impl Cmds {
    pub fn parse() -> Cmds {
        let matcher = Cmds::create_app().get_matches();

        if let Some(name) = matcher.value_of("server") {
            Cmds {
                cmd: Cmd::Server {
                    name: ServerCmdName::from(name),
                    config: matcher.value_of("config").unwrap().to_owned(),
                    port: matcher
                        .value_of("port")
                        .map(|s| s.parse::<u16>().unwrap())
                        .unwrap(),
                },
            }
        } else {
            Cmds { cmd: Cmd::Unknown }
        }
    }

    pub fn runnable_cmd(self) -> Box<dyn RunnableCmd> {
        match self.cmd {
            Cmd::Server {
                name: Start,
                config,
                port,
            } => Box::new(StartupCmd::new(config, port)),
            _ => Box::new(RunnableUnknown {
                app: Cmds::create_app(),
            }),
        }
    }

    fn create_app() -> App<'static, 'static> {
        App::new("surl")
            .version("0.1.0")
            .author("zhxiaog <zhxiaog@outlook.com>")
            .about("mock servers with `curl` like cmds")
            .arg(
                Arg::with_name("server")
                    .short("s")
                    .long("server")
                    .value_name("cmd")
                    .help("run server directives: start|stop|reload|show|watch")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .value_name("port")
                    .default_value("7575")
                    .help("deamon http server listen port")
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("config")
                    .short("f")
                    .long("config")
                    .value_name("config.json")
                    .default_value("./config.json")
                    .help("configuration file path")
                    .required(false)
                    .takes_value(true),
            )
    }
}
