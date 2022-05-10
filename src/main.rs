use actix_web:: {
    App, HttpServer
};
use aac_irish_local::services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(services::root_service::get)
            .service(services::text_correction_service::get)
            .service(services::voice_service::get)
    }).bind(("0.0.0.0", 8080))?.run().await
}