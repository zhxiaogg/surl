use crate::cmds::cmd::HttpServiceInfo;
use crate::utils::http::*;
use hyper::{Body, Error, Method, Request, Response, Server as HyperServer, StatusCode};
use std::collections::linked_list::LinkedList;

pub struct ServiceContainer {
    service_infos: LinkedList<HttpServiceInfo>,
}

impl ServiceContainer {
    pub fn new() -> ServiceContainer {
        ServiceContainer {
            service_infos: LinkedList::new(),
        }
    }

    pub fn add_service(&mut self, service_info: HttpServiceInfo) -> () {
        self.service_infos.push_back(service_info);
    }

    pub async fn process(&mut self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match self
            .service_infos
            .iter()
            .find(|&service| ServiceContainer::service_match(service, &req))
        {
            Some(service) => Ok(Response::builder()
                .status(StatusCode::OK)
                .body(Body::from(service.url.as_str().to_owned()))
                .unwrap()),
            _ => not_found(),
        }
    }

    fn service_match(service: &HttpServiceInfo, req: &Request<Body>) -> bool {
        let method_match = service.method.to_string() == req.method().as_str();
        method_match
    }
}
