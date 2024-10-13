use actix_web::{HttpRequest, HttpResponse, Responder};

#[actix_web::get("/")]
pub async fn root(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().body("Hello worldsss!")
}
