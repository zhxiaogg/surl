#[derive(Debug)]
pub enum Cmd {
    ServerCmd {
        name: ServerCmdName,
        config: String,
        port: u16,
    },
    HttpCmd {
        method: HttpMethod,
        url: String,
        response: Option<String>,
    },
    Unknown,
}

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Update,
    Delete,
    Unknown,
}

impl HttpMethod {
    pub fn from(name: &str) -> HttpMethod {
        use HttpMethod::*;
        match name.to_uppercase().as_ref() {
            "GET" => Get,
            "POST" => Post,
            "UPDATE" => Update,
            "DELETE" => Delete,
            _ => Unknown,
        }
    }
}

#[derive(Debug)]
pub enum ServerCmdName {
    Start,
    Unknown,
}

impl ServerCmdName {
    pub fn from(name: &str) -> ServerCmdName {
        match name {
            "start" => ServerCmdName::Start,
            _ => ServerCmdName::Unknown,
        }
    }
}
