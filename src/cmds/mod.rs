pub mod cmd;
mod runnable;

use clap::App;
use clap::Arg;

use runnable::StartupCmd;

use cmd::ServerCmdName::*;
use cmd::*;
use runnable::*;

pub struct Cmds {
    pub cmd: Cmd,
}

impl Cmds {
    pub fn parse() -> Cmds {
        let matcher = Cmds::create_app().get_matches();

        if let Some(name) = matcher.value_of("server") {
            Cmds {
                cmd: Cmd::ServerCmd {
                    name: ServerCmdName::from(name),
                    config: matcher.value_of("config").unwrap().to_owned(),
                    port: matcher
                        .value_of("port")
                        .map(|s| s.parse::<u16>().unwrap())
                        .unwrap(),
                },
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
        app.arg(
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
