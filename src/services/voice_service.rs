use actix_web::{HttpResponse, Responder, get};

#[get("/voice")]
pub async fn get() -> impl Responder {
    HttpResponse::NotFound().body("Not yet implemented.")
}