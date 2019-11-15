use super::tpl::{Renderer, RequestContext};
use crate::cmds::http::{HttpServiceId, HttpServiceInfo};
use crate::utils::http::*;
use hyper::{Body, Request, Response, StatusCode};
use std::collections::HashMap;

pub struct ServiceContainer {
    service_infos: HashMap<HttpServiceId, HttpServiceInfo>,
}

impl ServiceContainer {
    pub fn new() -> ServiceContainer {
        ServiceContainer {
            service_infos: HashMap::new(),
        }
    }

    pub fn add_service(&mut self, service_info: HttpServiceInfo) -> () {
        self.service_infos.insert(service_info.id(), service_info);
    }

    pub async fn process(&mut self, req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
        match self.find_service(&req) {
            Some(service) => {
                let ctx = RequestContext::new(service, req).await;
                self.create_response(service, &ctx)
            }
            _ => not_found(),
        }
    }

    fn find_service(&self, req: &Request<Body>) -> Option<&HttpServiceInfo> {
        let id = HttpServiceId::from(&req);
        match self.service_infos.get(&id) {
            i @ Some(_) => i,
            None => self
                .service_infos
                .values()
                .find(|s| s.id_for_req(req) == s.id()),
        }
    }

    fn create_response(
        &self,
        service: &HttpServiceInfo,
        ctx: &RequestContext,
    ) -> Result<Response<Body>, hyper::Error> {
        let mut b = Response::builder();
        for (k, v) in service.headers.iter() {
            b.header(k.trim(), v.as_ref().map_or("", String::as_ref).trim());
        }
        Ok(b.status(StatusCode::OK)
            .body(self.body(service, ctx))
            .unwrap())
    }

    fn body(&self, service: &HttpServiceInfo, ctx: &RequestContext) -> Body {
        let response = service.response.as_ref().map_or("", String::as_str);

        let renderer = Renderer::new();
        let resp = renderer.render(response, &ctx);
        Body::from(resp.unwrap().to_owned())
    }
}
