# Compute@Edge default starter kit for Rust

Get to know the Fastly Compute@Edge environment with a basic starter that demonstrates routing, simple synthetic responses, and overriding caching rules.

**For more details about this and other starter kits for Compute@Edge, see the [Fastly Developer Hub](https://developer.fastly.com/solutions/starters/)**.

## Features

- Add headers to an inbound request
- Allow only requests with particular HTTP methods
- Match request URL path and methods for routing
- Build synthetic responses at the edge
- Send requests to a backend
- Override caching policy response headers

## Understanding the code

This starter is intentionally lightweight, and requires no dependencies aside from the [`fastly`](https://docs.rs/fastly) crate. It will help you understand the basics of processing requests at the edge using Fastly. This starter includes implementations of common patterns explained in our [using Compute@Edge](https://developer.fastly.com/learning/compute/rust/) and [VCL migration](https://developer.fastly.com/learning/compute/migrate/) guides.

The path `/backend` will attempt to send a request to a backend called "backend_name". If the service you have installed this starter on doesn't have a backend defined, use the [`fastly backend create`](https://developer.fastly.com/reference/cli/backend/create/) command to create one. Modify the following lines of the starter to use the name of the backend you created:

```rust
/// The name of a backend server associated with this service.
///
/// This should be changed to match the name of your own backend. See the the `Hosts` section of
/// the Fastly WASM service UI for more information.
const BACKEND_NAME: &str = "backend_name";

/// The name of a second backend associated with this service.
const OTHER_BACKEND_NAME: &str = "other_backend_name";
```

The starter uses two backends, so if you want to, go ahead and create two backends using the CLI and then modify both names here. You should now have a Fastly service running on Compute@Edge that can talk to your backends, and generate synthetic responses at the edge.

## Security issues

Please see [SECURITY.md](SECURITY.md) for guidance on reporting security-related issues.
