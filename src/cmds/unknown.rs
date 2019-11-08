use super::RunnableCmd;
use clap::App;

pub struct RunnableUnknown {
    pub app: App<'static, 'static>,
}

impl RunnableUnknown {
    pub fn new(app: App<'static, 'static>) -> RunnableUnknown {
        RunnableUnknown { app }
    }
}

impl RunnableCmd for RunnableUnknown {
    fn run(&mut self) -> Result<(), String> {
        eprintln!("unrecognized cmd.");
        self.app.print_help().unwrap();
        Result::Ok(())
    }
}
