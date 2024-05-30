use std::io::Write;

use http::httprequest::HttpRequest;

use crate::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) {
        match req.method {
            http::httprequest::Method::Get => match &req.resource {
                http::httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split("/").collect();
                    match route[1] {
                        "api" => {
                            let resp = WebServiceHandler::handler(&req);
                            let _ = resp.send_response(stream);
                        }
                        _ => {
                            let resp = StaticPageHandler::handler(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },
            _ => {
                let resp = PageNotFoundHandler::handler(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
