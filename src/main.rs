use fastly::http::{Method, StatusCode};
use fastly::{downstream_request, Body, Error, Request, RequestExt, Response, ResponseExt};
use std::convert::TryFrom;

const ONE_MINUTE_TTL: i32 = 60;
const NO_CACHE_TTL: i32 = -1;

/// Handle the downstream request from the client.
///
/// This function accepts a Request<Body> and returns a Response<Body>. It could
/// be used to route based on the request properties (such as method or path),
/// send the request to a backend, make completely new requests and/or generate
/// synthetic responses.
fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    // Pattern match on the request method and path.
    match (req.method(), req.uri().path()) {
        // If request is `GET /`.
        (&Method::GET, "/") => {
            // Request handling logic could go here...
            // Such as send the request to an origin backend and then cache the
            // response for one minute.
            req.send("backend-name", ONE_MINUTE_TTL)
        }
        // If request path starts with `/other/`.
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

fn main() -> Result<(), Error> {
    // Get the downstream request from the client.
    let req = downstream_request()?;
    // Pass the request to the request handler and return a response.
    let resp = handle_request(req)?;
    // Send the response downstream to the client.
    resp.send_downstream()?;
    Ok(())
}
