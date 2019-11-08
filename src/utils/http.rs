use hyper::{Body, Response, StatusCode};

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
