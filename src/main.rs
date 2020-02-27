use fastly::http::{HeaderValue, Method, StatusCode};
use fastly::{downstream_request, Body, Error, Request, RequestExt, Response, ResponseExt};
use std::convert::TryFrom;

const ONE_MINUTE_TTL: i32 = 60;
const NO_CACHE_TTL: i32 = -1;
const VALID_METHODS: [Method; 6] = [
    Method::HEAD,
    Method::GET,
    Method::POST,
    Method::PUT,
    Method::PATCH,
    Method::DELETE,
];

/// Handle the downstream request from the client.
///
/// This function accepts a Request<Body> and returns a Response<Body>. It could
/// be used to route based on the request properties (such as method or path),
/// send the request to a backend, make completely new requests and/or generate
/// synthetic responses.
fn handle_request(mut req: Request<Body>) -> Result<Response<Body>, Error> {
    // Make any desired changes to the client request
    req.headers_mut()
        .insert("Host", HeaderValue::from_static("example.com"));

    if !(VALID_METHODS.contains(req.method())) {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::try_from("Only GET and HEAD requests are allowed")?)?);
    }

    // Pattern match on the request method and path.
    match (req.method(), req.uri().path()) {
        // If request is a `GET` to the `/` path, send to a named backend
        (&Method::GET, "/") => {
            // Request handling logic could go here...
            // Eg. send the request to an origin backend and then cache the
            // response for one minute.
            req.send("backend-name", ONE_MINUTE_TTL)
        }

        // If request is a `GET` to a path starting with `/other/`.
        (&Method::GET, path) if path.starts_with("/other/") => {
            // Send request to a different backend and don't cache response.
            req.send("other-backend-name", NO_CACHE_TTL)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::try_from("The page you requested could not be found")?)?),
    }
}

/// The entrypoint for your application
///
/// This function is triggered when your service receives a client request, and
/// should ultimately call `send_downstream` on a fastly::Response to deliver an
/// HTTP response to the client.
fn main() -> Result<(), Error> {
    let req = downstream_request()?;
    match handle_request(req) {
        Ok(resp) => resp.send_downstream()?,
        Err(e) => {
            let mut resp = Response::new(Vec::from(e.msg));
            *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            resp.send_downstream()?;
        }
    }
    Ok(())
}
