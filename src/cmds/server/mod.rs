mod startup;
mod stop;

use super::RunnableCmd;
pub use startup::StartupCmd;
pub use stop::StopCmd;

pub fn create_server_cmd(name: Option<String>, port: u16) -> Box<dyn RunnableCmd> {
    match name.as_ref().map(|s| s.as_ref()) {
        Some("start") => Box::new(StartupCmd::new("".to_owned(), port)),
        Some("stop") => Box::new(StopCmd::new("".to_owned(), port)),
        unknown => Box::new(UnknownServerCmd::new(unknown.map(|s| s.to_owned()))),
    }
}

struct UnknownServerCmd {
    name: Option<String>,
}

impl UnknownServerCmd {
    fn new(name: Option<String>) -> UnknownServerCmd {
        UnknownServerCmd { name }
    }
}

impl RunnableCmd for UnknownServerCmd {
    fn run(&mut self) -> Result<(), String> {
        match self.name {
            Some(ref cmd) => eprintln!("unrecognized server cmd: {}", cmd),
            None => eprintln!("no server cmd specified."),
        };
        Ok(())
    }
}
