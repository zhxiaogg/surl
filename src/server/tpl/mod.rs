use crate::utils::http::*;
use crate::utils::tpl::{to_json, unix_timestamp};
use handlebars::Handlebars;
use http::{Request, Uri};
use hyper::Body;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Serialize)]
pub struct RequestContext {
    body: Option<Value>,
    path: BTreeMap<String, String>,
    params: BTreeMap<String, String>,
}

impl RequestContext {
    pub async fn new(request: Request<Body>) -> RequestContext {
        let (parts, mut body) = request.into_parts();
        let body = body_to_str(&mut body).await;
        let body = body
            .map(|ref s| serde_json::from_str::<Value>(s).ok())
            .flatten();
        let params = decode_query_params(&parts.uri);
        let path_variables = decode_path_variables(&parts.uri);
        RequestContext {
            body: body,
            path: path_variables,
            params: params,
        }
    }
}

fn decode_path_variables(uri: &Uri) -> BTreeMap<String, String> {
    BTreeMap::new()
}

pub struct Renderer {
    handlebar: Handlebars,
}

impl Renderer {
    pub fn new() -> Renderer {
        let mut h = Handlebars::new();
        h.register_helper("json", Box::new(to_json));
        h.register_helper("timestamp", Box::new(unix_timestamp));
        Renderer { handlebar: h }
    }
    pub fn render(&self, tpl: &str, ctx: &RequestContext) -> Result<String, String> {
        self.handlebar
            .render_template(tpl, ctx)
            .map_err(|e| format!("templating error: {}", e))
    }
}

mod test {
    use super::*;
    use serde_json::from_str;

    #[test]
    fn can_render_a_plain_text() {
        let renderer = Renderer::new();
        let ctx = RequestContext {
            body: None,
            path: BTreeMap::new(),
            params: BTreeMap::new(),
        };
        let tpl = "a simple test";
        let r = renderer.render(tpl, &ctx);
        assert_eq!(tpl, r.unwrap());
    }

    #[test]
    fn can_render_with_request_params() {
        let renderer = Renderer::new();
        let mut params = BTreeMap::new();
        params.insert("answer".to_owned(), "42".to_owned());
        let ctx = RequestContext {
            body: None,
            path: BTreeMap::new(),
            params: params,
        };
        let tpl = "the answer = {{ params.answer }}";
        let r = renderer.render(tpl, &ctx);
        assert_eq!("the answer = 42", r.unwrap());
    }

    #[test]
    fn can_render_with_json_request_body() {
        let renderer = Renderer::new();
        let ctx = RequestContext {
            body: Some(from_str("{\"answer\": 42}").ok()).flatten(),
            path: BTreeMap::new(),
            params: BTreeMap::new(),
        };
        let tpl = "the answer = {{ body.answer }}";
        let r = renderer.render(tpl, &ctx);
        assert_eq!("the answer = 42", r.unwrap());
    }

    #[test]
    fn can_render_with_json_objects() {
        let renderer = Renderer::new();
        let ctx = RequestContext {
            body: Some(from_str("{\"answer\":42}").ok()).flatten(),
            path: BTreeMap::new(),
            params: BTreeMap::new(),
        };
        let tpl = "{{ json body }}";
        let r = renderer.render(tpl, &ctx);
        assert_eq!("{\"answer\":42}", r.unwrap());
    }
}
