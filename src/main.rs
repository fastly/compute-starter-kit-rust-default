use fastly::http::{HeaderValue, Method, StatusCode};
use fastly::request::CacheOverride;
use fastly::{Body, Error, Request, RequestExt, Response, ResponseExt};
use std::convert::TryFrom;

const VALID_METHODS: [Method; 3] = [Method::HEAD, Method::GET, Method::POST];

/// The entrypoint for your application.
///
/// Unlike a normal `main`, a function annotated with `#[fastly::main]` accepts a `Request<Body>`
/// and returns a `Response<Body>`. It could be used to route based on the request properties (such
/// as method or path), send the request to a backend, make completely new requests and/or generate
/// synthetic responses.
#[fastly::main]
fn main(mut req: Request<Body>) -> Result<Response<Body>, Error> {
    // Make any desired changes to the client request
    req.headers_mut()
        .insert("Host", HeaderValue::from_static("example.com"));

    if !(VALID_METHODS.contains(req.method())) {
        return Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::try_from("This method is not allowed")?)?);
    }

    // Pattern match on the request method and path.
    match (req.method(), req.uri().path()) {
        // If request is a `GET` to the `/` path, send a default response.
        (&Method::GET, "/") => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::try_from("Welcome to Fastly Compute@Edge!")?)?),

        // If request is a `GET` to the `/backend` path, send to a named backend.
        (&Method::GET, "/backend") => {
            // Request handling logic could go here...
            // Eg. send the request to an origin backend and then cache the
            // response for one minute.
            req.set_cache_override(CacheOverride::ttl(60));
            req.send("backend_name")
        }

        // If request is a `GET` to a path starting with `/other/`.
        (&Method::GET, path) if path.starts_with("/other/") => {
            // Send request to a different backend and don't cache response.
            req.set_cache_override(CacheOverride::Pass);
            req.send("other_backend_name")
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::try_from("The page you requested could not be found")?)?),
    }
}
