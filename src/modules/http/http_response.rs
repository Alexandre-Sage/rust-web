extern crate iron;
extern crate mime;
use crate::status::Status;
use iron::{headers::ContentType, prelude::*};
use mime::Mime;
pub struct HttpResponse {
    response: Response,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self {
            response: Response::new(),
        }
    }
    pub fn set_html_headers(mut self) -> Self {
        self.response.headers.set(ContentType(Mime(
            mime::TopLevel::Text,
            mime::SubLevel::Html,
            vec![],
        )));
        self
    }
    pub fn set_json_headers(mut self) -> Self {
        self.response.headers.set(ContentType(Mime(
            mime::TopLevel::Application,
            mime::SubLevel::Json,
            vec![],
        )));
        self
    }
    pub fn set_status(mut self, status: Status) -> Self {
        self.response.set_mut(status);
        self
    }
    pub fn set_response_body(mut self, body: String) -> Self {
        self.response.set_mut(body);
        self
    }
    pub fn get_instance(self) -> IronResult<Response> {
        Ok(self.response)
    }
}
