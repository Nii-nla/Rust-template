//a web microservice for calculating multiple types of operations
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Calculator Microservice!")
}

#[get("/add/{a}/{b}")]
async fn add(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::add(info.0, info.1);
    HttpResponse::Ok().body(format!("Result: {}", res))
}

#[get("/subtract/{a}/{b}")]
async fn subtract(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::subtract(info.0, info.1);
    HttpResponse::Ok().body(format!("Result: {}", res))
}

#[get("/multiply/{a}/{b}")]
async fn multiply(info: web::Path<(i32, i32)>) -> impl Responder {
    let res = calc::multiply(info.0, info.1);
    HttpResponse::Ok().body(format!("Result: {}", res))
}

#[get("/divide/{a}/{b}")]
async fn divide(info: web::Path<(i32, i32)>) -> impl Responder {
    if info.1 == 0 {
        return HttpResponse::BadRequest().body("Cannot divide by zero!");
    }
    let res = calc::divide(info.0, info.1);
    HttpResponse::Ok().body(format!("Result: {}", res))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
// This code is a simple web microservice built using Actix-web.
