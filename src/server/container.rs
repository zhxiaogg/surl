use crate::cmds::http::HttpServiceInfo;
use crate::utils::http::*;
use http::Uri;
use hyper::{Body, Request, Response, StatusCode};
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
            Some(service) => self.create_response(service),
            _ => not_found(),
        }
    }

    fn create_response(&self, service: &HttpServiceInfo) -> Result<Response<Body>, hyper::Error> {
        let mut b = Response::builder();
        for (k, v) in service.headers.iter() {
            b.header(k.trim(), v.as_ref().map_or("", String::as_ref).trim());
        }
        Ok(b.status(StatusCode::OK).body(self.body(service)).unwrap())
    }

    fn body(&self, service: &HttpServiceInfo) -> Body {
        let response = service
            .response
            .as_ref()
            .map_or("", String::as_str)
            .to_owned();
        Body::from(response)
    }

    fn service_match(service: &HttpServiceInfo, req: &Request<Body>) -> bool {
        let method_match = service.method.to_string() == req.method().as_str();
        let uri = if service.url.contains("://") {
            service.url.parse::<Uri>().unwrap()
        } else {
            let url = format!("http://{}", service.url);
            let s: &str = url.as_ref();
            s.parse::<Uri>().unwrap()
        };
        let uri_match = uri.path() == req.uri().path();
        method_match && uri_match
    }
}
