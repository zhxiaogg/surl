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
    pub fn new(request: &Request<Body>) -> RequestContext {
        RequestContext {
            body: None,
            path: decode_path_variables(request.uri()),
            params: decode_query_params(request.uri()),
        }
    }
}

fn decode_path_variables(uri: &Uri) -> BTreeMap<String, String> {
    BTreeMap::new()
}

fn decode_query_params(uri: &Uri) -> BTreeMap<String, String> {
    match uri.query() {
        Some(query) => {
            let mut m = BTreeMap::new();
            for (k, v) in query
                .split("&")
                .filter(|s| s.trim().len() > 0)
                .map(|s| split_str_to_pair(s, "="))
            {
                m.insert(k, v);
            }
            m
        }
        None => BTreeMap::new(),
    }
}

fn split_str_to_pair(s: &str, splitter: &str) -> (String, String) {
    let vec = s.splitn(2, splitter).collect::<Vec<&str>>();
    let left = vec.get(0).map(|s| s.to_owned()).unwrap().to_owned();
    let right = vec.get(1).map(|s| s.to_owned()).unwrap_or("").to_owned();
    (left, right)
}

pub struct Renderer {
    handlebar: Handlebars,
}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {
            handlebar: Handlebars::new(),
        }
    }
    pub fn render(&self, tpl: &str, ctx: &RequestContext) -> Result<String, String> {
        self.handlebar
            .render_template(tpl, ctx)
            .map_err(|e| format!("templating error: {}", e))
    }
}

mod test {
    use super::*;
    use serde_json::json;

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
            body: Some(json!({"answer": 42})),
            path: BTreeMap::new(),
            params: BTreeMap::new(),
        };
        let tpl = "the answer = {{ body.answer }}";
        let r = renderer.render(tpl, &ctx);
        assert_eq!("the answer = 42", r.unwrap());
    }
}
