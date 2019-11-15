use http::{HeaderMap, HeaderValue};
use hyper::{Body, Response, StatusCode, Uri};
use std::collections::BTreeMap;

pub async fn body_to_str(body: &mut Body) -> Option<String> {
    match body.next().await {
        Some(Ok(chunk)) => {
            let bytes = chunk.into_bytes();
            let json_body = String::from_utf8(bytes.as_ref().to_vec());
            match json_body {
                Ok(json) => Some(json),
                Err(e) => {
                    eprintln!("body_to_str failed: {}", e);
                    None
                }
            }
        }
        _ => None,
    }
}

pub fn ok() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}

pub fn bad_request() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::empty())
        .unwrap())
}

pub fn not_found() -> Result<Response<Body>, hyper::Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap())
}

pub fn decode_query_params(uri: &Uri) -> BTreeMap<String, String> {
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

pub fn decode_headers(header_map: &HeaderMap<HeaderValue>) -> BTreeMap<String, Option<String>> {
    let mut headers: BTreeMap<String, Option<String>> = BTreeMap::new();
    for (name, value) in header_map.iter() {
        headers.insert(
            name.as_str().to_owned(),
            value.to_str().ok().map(str::to_owned),
        );
    }
    headers
}
