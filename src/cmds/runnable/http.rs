use crate::cmds::cmd::*;
use super::RunnableCmd;

pub struct RunnableHttpCmd {
    method: HttpMethod,
    url:String,
    response:Option<String>,
}

impl RunnableHttpCmd {
    pub fn new(method:HttpMethod, url:String, response:Option<String>) -> RunnableHttpCmd {
        RunnableHttpCmd {
            method,url,response
        }
    }
}

impl RunnableCmd for RunnableHttpCmd {
    fn run(&mut self) -> Result<(), String> {
        // TODO: create a new HTTP mock in server side
        println!("{:?} {} {:?}", self.method,self.url,self.response);
        Ok(())
    }
}