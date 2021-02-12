//! Utilities for working with HTTP requests and responses.

use hyper::{Body, Response, StatusCode};
use std::collections::HashMap;

/// Create an HTTP 404 response
pub(crate) fn not_found() -> Response<Body> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    not_found
}

/// Create an HTTP 500 response
pub(crate) fn internal_error(msg: &str) -> Response<Body> {
    println!("HTTP 500 error: {}", msg);
    let mut res = Response::new(Body::from(msg.to_owned()));
    *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    res
}

pub(crate) fn parse_cgi_headers(headers: String) -> HashMap<String, String> {
    let mut map = HashMap::new();
    headers.trim().split('\n').for_each(|h| {
        let parts: Vec<&str> = h.splitn(2, ':').collect();
        if parts.len() != 2 {
            println!("corrupt header: {}", h);
            return;
        }
        map.insert(parts[0].trim().to_owned(), parts[1].trim().to_owned());
    });
    map
}
