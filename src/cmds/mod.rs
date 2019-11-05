pub mod cmd;
mod runnable;

use clap::{App, Arg, SubCommand};

use runnable::{StartupCmd, StopCmd};

use cmd::ServerCmdName::*;
use cmd::*;
use runnable::*;

pub struct Cmds {
    pub cmd: Cmd,
}

impl Cmds {
    pub fn parse() -> Cmds {
        let matcher = Cmds::create_app().get_matches();

        if let Some(srever_cmd) = matcher.subcommand_matches("server") {
            if let Some(name) = srever_cmd.value_of("cmd") {
                Cmds {
                    cmd: Cmd::ServerCmd {
                        name: ServerCmdName::from(name),
                        config: "".to_owned(),
                        port: srever_cmd
                            .value_of("port")
                            .map(|s| s.parse::<u16>().unwrap())
                            .unwrap(),
                    },
                }
            } else {
                // unknown server cmd
                Cmds { cmd: Cmd::Unknown }
            }
        } else if let Some(method) = matcher.value_of("request") {
            Cmds {
                cmd: Cmd::HttpCmd {
                    method: HttpMethod::from(method),
                    url: matcher.value_of("url").unwrap().to_owned(),
                    response: matcher.value_of("data").map(|s| s.to_owned()),
                },
            }
        } else {
            Cmds { cmd: Cmd::Unknown }
        }
    }

    pub fn runnable_cmd(self) -> Box<dyn RunnableCmd> {
        match self.cmd {
            Cmd::ServerCmd {
                name: Start,
                config,
                port,
            } => Box::new(StartupCmd::new(config, port)),
            Cmd::ServerCmd {
                name: Stop,
                config,
                port,
            } => Box::new(StopCmd::new(config, port)),
            Cmd::HttpCmd {
                method,
                url,
                response,
            } => Box::new(RunnableHttpCmd::new(HttpServiceInfo::new(
                method, url, response,
            ))),
            c => {
                println!("unrecognized cmd: {:?}", c);
                Box::new(RunnableUnknown {
                    app: Cmds::create_app(),
                })
            }
        }
    }

    fn create_app() -> App<'static, 'static> {
        let app = App::new("surl")
            .version("0.1.0")
            .author("zhxiaog <zhxiaog@outlook.com>")
            .about("mock servers with `curl` like cmds");
        let server_app = Cmds::add_server_directives(app);
        let http_app = Cmds::add_http_directives(server_app);
        http_app
    }

    fn add_http_directives(app: App<'static, 'static>) -> App<'static, 'static> {
        app.arg(
            Arg::with_name("request")
                .short("X")
                .long("request")
                .value_name("METHOD")
                .help("http methods: GET|POST|UPDATE|DELETE")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .value_name("url")
                .help("mocking url")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("data")
                .short("d")
                .long("data")
                .value_name("DATA")
                .help("expected response data, could be string or file (starts with `@`)")
                .required(false)
                .takes_value(true),
        )
    }

    fn add_server_directives(app: App<'static, 'static>) -> App<'static, 'static> {
        let sub_cmd = SubCommand::with_name("server")
            .about("talk to deamon server")
            .version("0.1.0")
            .arg(
                Arg::with_name("cmd")
                    .value_name("CMD")
                    .takes_value(true)
                    .help("run server cmds: start|stop"),
            )
            .arg(
                Arg::with_name("port")
                    .short("p")
                    .long("port")
                    .takes_value(true)
                    .default_value("8080")
                    .value_name("PORT")
                    .required(false)
                    .help("binding port"),
            );
        app.subcommand(sub_cmd)
    }
}
