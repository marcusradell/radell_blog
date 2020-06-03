use actix_web::{web, App, HttpServer};
mod discover;
mod status;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(discover::handler))
            .service(web::resource("/status").to(status::handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
