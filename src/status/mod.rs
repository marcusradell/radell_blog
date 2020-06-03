use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;
use futures::future::{ready, Ready};
use serde::Serialize;

#[derive(Serialize)]
struct Status {
    alive: bool,
    ready: bool,
}

impl Responder for Status {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Self::Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

pub async fn handler() -> impl Responder {
    Status {
        alive: true,
        ready: true,
    }
}
