use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello actix")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 5000;
    HttpServer::new(||
        App::new().service(greet)
    ).bind(("127.0.0.1", port)).unwrap().run().await
}
