use super::RunnableCmd;
use std::fs::File;

use crate::server::HttpServer;
use daemonize::Daemonize;

pub struct StartupCmd {
    config: String,
    port: u16,
}

impl StartupCmd {
    pub fn new(config: String, port: u16) -> StartupCmd {
        StartupCmd { config, port }
    }

    fn run_http_server(port: u16) -> () {
        let mut server = HttpServer::new("127.0.0.1", port);
        if let Err(e) = server.start_foreground() {
            eprintln!("server failed: {}", e);
        }
    }
}

impl RunnableCmd for StartupCmd {
    fn run(&mut self) -> Result<(), String> {
        let stdout = File::create("/tmp/daemon.out").unwrap();
        let stderr = File::create("/tmp/daemon.err").unwrap();

        let port = self.port;
        let daemonize = Daemonize::new()
            .pid_file("/tmp/test.pid") // Every method except `new` and `start`
            .chown_pid_file(true) // is optional, see `Daemonize` documentation
            .working_directory("/tmp") // for default behaviour.
            .umask(0o777) // Set umask, `0o027` by default.
            .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
            .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
            .privileged_action(move || {
                StartupCmd::run_http_server(port);
            });

        match daemonize.start() {
            Ok(_) => println!("Success, daemonized"),
            Err(e) => eprintln!("Error, {}", e),
        }
        Ok(())
    }
}
