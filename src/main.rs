use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    match req.get_method() {
        // Block requests with unexpected methods
        &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, PURGE")
                .with_body_text_plain("This method is not allowed\n"))
        }

        // Let any other requests through
        _ => (),
    };

    match req.get_path() {
        "/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/index.html")))
        }
        "/index.xml" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_XML)
                .with_body(include_str!("static_site/hosfe.lt/public/index.xml"))) 
        },
        "/sitemap.xml" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_XML)
                .with_body(include_str!("static_site/hosfe.lt/public/sitemap.xml"))) 
        }
        "/posts/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/posts/index.html"))) 
        },
        "/posts/index.xml" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_XML)
                .with_body(include_str!("static_site/hosfe.lt/public/posts/index.xml"))) 
        }
        "/about/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/about/index.html"))) 
        },
        "/resume/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/resume/index.html"))) 
        },
        "/css/style.min.dbbe08cb3b07bbce02de1a13a57d4221bb75487e75b0d1a5196a5353f7135921.css" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "https://www.hosfe.lt")
                .with_content_type(mime::TEXT_CSS_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/css/style.min.dbbe08cb3b07bbce02de1a13a57d4221bb75487e75b0d1a5196a5353f7135921.css")))
        }
        "/js/bundle.min.038214de9d568246fadcfeb06c69349925de3209f332ec123861b6aa031d63c6.js" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::APPLICATION_JAVASCRIPT_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/js/bundle.min.038214de9d568246fadcfeb06c69349925de3209f332ec123861b6aa031d63c6.js")))
        },
        "/js/link-share.min.24409a4f6e5537d70ffc55ec8f9192208d718678cb8638585342423020b37f39.js" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::APPLICATION_JAVASCRIPT_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/js/link-share.min.24409a4f6e5537d70ffc55ec8f9192208d718678cb8638585342423020b37f39.js")))
        },
        // blog posts go after here
        "/posts/story/" => {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/posts/story/index.html")))
        },
        // Catch all other requests and return a 404.
        _ =>  {
            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::TEXT_HTML_UTF_8)
                .with_body(include_str!("static_site/hosfe.lt/public/404.html")))
        },
    }
}
