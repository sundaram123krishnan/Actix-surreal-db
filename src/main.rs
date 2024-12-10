use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder, post, patch};

#[get("/greet")]
async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello actix")
}

#[post("/ex_post")]
async fn ex_post() -> impl Responder {
    HttpResponse::Ok().body("hello post")
}

#[patch("/ex_update/{id}")]
async fn ex_update() -> impl Responder {
    HttpResponse::Ok().body("hello update")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 5000;
    println!("Running on port {}...", port);
    HttpServer::new(||
        App::new().service(greet).service(ex_post).service(ex_update)
    ).bind(("127.0.0.1", port)).unwrap().run().await
}


