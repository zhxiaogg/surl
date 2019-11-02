mod http;
mod startup;

use clap::App;

pub use http::RunnableHttpCmd;
pub use startup::StartupCmd;

pub trait RunnableCmd {
    fn run(&mut self) -> Result<(), String>;
}

pub struct RunnableUnknown {
    pub app: App<'static, 'static>,
}

impl RunnableCmd for RunnableUnknown {
    fn run(&mut self) -> Result<(), String> {
        self.app.print_help().unwrap();
        Result::Ok(())
    }
}
