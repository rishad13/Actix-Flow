use actix_web::{get, HttpResponse, Responder};


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json("hello rustacean! 🦀")
}


#[get("/test")]
async fn test() -> impl Responder {
    HttpResponse::Ok().json("hello rustacean! test 🦀")
}