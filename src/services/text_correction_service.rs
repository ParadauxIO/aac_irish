use actix_web::{HttpResponse, Responder, get};

#[get("/corrector")]
pub async fn get() -> impl Responder {
    HttpResponse::NotFound().body("Not yet implemented.")
}