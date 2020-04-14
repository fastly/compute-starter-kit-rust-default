//! Default Compute@Edge template program.

use fastly::http::response::Builder as ResponseBuilder;
use fastly::http::{HeaderValue, Method, StatusCode};
use fastly::request::CacheOverride;
use fastly::{downstream_request, Body, Error, Request, RequestExt, Response, ResponseExt};
use std::convert::TryFrom;

/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "backend_name";

/// The name of a second backend associated with this service.
const OTHER_BACKEND_NAME: &str = "other_backend_name";

/// The entrypoint for your application.
///
/// This function is triggered when your service receives a client request. This function can call
/// [`fastly::downstream_request`][dsreq] to get the client request, and should ultimately call
/// [`fastly::send_downstream`][send] on a [`fastly::Response`][resp] to deliver an HTTP response
/// to the client.
///
/// If `main` returns an error, a [500 Internal Server Error][err] will be delivered to the client.
///
/// [dsreq]: ../fastly/request/fn.downstream_request.html
/// [err]: https://tools.ietf.org/html/rfc7231#section-6.6.1
/// [resp]: ../fastly/struct.Response.html
/// [send]: ../fastly/response/trait.ResponseExt.html#method.send_downstream
fn main() -> Result<(), Error> {
    let req = downstream_request()?;
    let resp = match handle_request(req) {
        Ok(resp) => resp,
        Err(e) => {
            let body = Body::try_from(e.to_string())?;
            ResponseBuilder::new()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(body)?
        }
    };
    resp.send_downstream()?;
    Ok(())
}

/// Handle the downstream request from the client.
///
/// This function accepts a [`Request<Body>`][req] and returns a [`Response<Body>`][resp]. It could
/// be used to route based on the request properties (such as method or path), send the request to
/// a backend, make completely new requests, and/or generate synthetic responses.
///
/// [req]: ../fastly/struct.Request.html
/// [resp]: ../fastly/struct.Response.html
fn handle_request(mut req: Request<Body>) -> Result<Response<Body>, Error> {
    // Make any desired changes to the client request
    req.headers_mut()
        .insert("Host", HeaderValue::from_static("example.com"));

    // We can filter requests that have unexpected methods.
    const VALID_METHODS: [Method; 3] = [Method::HEAD, Method::GET, Method::POST];
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
            req.send(BACKEND_NAME)
        }

        // If request is a `GET` to a path starting with `/other/`.
        (&Method::GET, path) if path.starts_with("/other/") => {
            // Send request to a different backend and don't cache response.
            req.set_cache_override(CacheOverride::Pass);
            req.send(OTHER_BACKEND_NAME)
        }

        // Catch all other requests and return a 404.
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::try_from("The page you requested could not be found")?)?),
    }
}
