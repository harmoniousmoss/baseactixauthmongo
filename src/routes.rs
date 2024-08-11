use crate::middleware::jwt_auth::JwtAuth;
use actix_web::{web, App, HttpResponse, HttpServer};

use crate::handlers::{signin, signup};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // User endpoints
        .service(web::resource("/signup").route(web::post().to(signup)))
        .service(web::resource("/signin").route(web::post().to(signin)));
}

pub fn configure_greet(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(|| async { HttpResponse::Ok().body("Hello Actix Mongo") })),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
            .configure(configure_greet)
            .wrap(JwtAuth) // Wrap the entire App with the middleware
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
