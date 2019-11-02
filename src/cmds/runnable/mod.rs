mod startup;
mod http;

use clap::App;

pub use startup::StartupCmd;
pub use http::RunnableHttpCmd;

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
