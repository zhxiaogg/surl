use clap::App;
use clap::Arg;

pub enum Cmd {
    Server {
        name: String,
        config: Option<String>,
    },
    Unknown {
        app: App<'static, 'static>,
    },
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
                    name: name.to_owned(),
                    config: matcher.value_of("config").map(|s| s.to_owned()),
                },
            }
        } else {
            Cmds {
                cmd: Cmd::Unknown {
                    app: Cmds::create_app(),
                },
            }
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
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("config")
                    .short("f")
                    .long("config")
                    .value_name("config.json")
                    .default_value("./config.json")
                    .required(false)
                    .help("configuration file path")
                    .takes_value(true),
            )
    }
}
