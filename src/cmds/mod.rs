use clap::{App, Arg, SubCommand};

mod server;
use server::create_server_cmd;
pub mod http;
use self::http::*;
mod unknown;
use unknown::RunnableUnknown;

pub trait RunnableCmd {
    fn run(&mut self) -> Result<(), String>;
}

pub fn parse_cmd() -> Box<dyn RunnableCmd> {
    let matcher = create_app().get_matches();

    if let Some(srever_cmd) = matcher.subcommand_matches("server") {
        let cmd_name_opt = srever_cmd.value_of("cmd").map(|s| s.to_owned());
        let port = srever_cmd
            .value_of("port")
            .map(|s| s.parse::<u16>().unwrap())
            .unwrap();
        create_server_cmd(cmd_name_opt, port)
    } else if let Some(method) = matcher.value_of("request") {
        let url = matcher.value_of("url").unwrap().to_owned();
        let response = matcher.value_of("data").map(|s| s.to_owned());
        let header = matcher.value_of("header");
        let http_service_info = HttpServiceInfo::new(method, url, response, header);
        Box::new(RunnableHttpCmd::new(http_service_info))
    } else {
        Box::new(RunnableUnknown::new(create_app()))
    }
}

fn create_app() -> App<'static, 'static> {
    let app = App::new("surl")
        .version("0.1.0")
        .author("zhxiaog <zhxiaog@outlook.com>")
        .about("mock servers with `curl` like cmds");
    let server_app = add_server_directives(app);
    let http_app = add_http_directives(server_app);
    http_app
}

fn add_http_directives(app: App<'static, 'static>) -> App<'static, 'static> {
    app.arg(
        Arg::with_name("request")
            .short("X")
            .long("request")
            .value_name("METHOD")
            .help("http methods: GET|POST|UPDATE|DELETE")
            .default_value("GET")
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
    .arg(
        Arg::with_name("header")
            .short("H")
            .long("header")
            .value_name("LINE")
            .help("response with custom headers")
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
