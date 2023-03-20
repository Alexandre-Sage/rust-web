extern crate iron;
extern crate mime;
use iron::{headers::ContentType, prelude::*, status::Status};
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
        return self;
    }
    pub fn set_json_headers(mut self) -> Self {
        self.response.headers.set(ContentType(Mime(
            mime::TopLevel::Application,
            mime::SubLevel::Json,
            vec![],
        )));
        return self;
    }
    pub fn set_status(mut self, status: Status) -> Self {
        self.response.set_mut(status);
        return self;
    }
    pub fn set_response_body<T: iron::modifier::Modifier<iron::Response>>(
        mut self,
        body: T,
    ) -> Self {
        self.response.set_mut::<T>(body);
        return self;
    }
    pub fn get_instance(self) -> IronResult<Response> {
        Ok(self.response)
    }
}
