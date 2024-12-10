mod models;

use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder, post, patch};
use actix_web::web::Path;
use actix_web::web::Json;
use crate::models::{AddTaskRequest, UpdateTask};

#[get("/get_tasks")]
async fn tasks() -> impl Responder {
    HttpResponse::Ok().body("Your tasks")
}

#[post("/add_task")]
async fn add_task(body: Json<AddTaskRequest>) -> impl Responder {
    HttpResponse::Ok().body(format!("Your title is: {}", body.title.clone()))
}

#[patch("/update_task/{uuid}")]
async fn update_task(update_task: Path<UpdateTask>) -> impl Responder {
    let uuid = update_task.into_inner().uuid;
    HttpResponse::Ok().body(format!("Updating the task with: {uuid}"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 5500;
    println!("Running on port {}...", port);
    HttpServer::new(||
        App::new().service(tasks).service(add_task).service(update_task)
    ).bind(("127.0.0.1", port)).unwrap().run().await
}


