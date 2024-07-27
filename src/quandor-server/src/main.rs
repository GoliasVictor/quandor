use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
struct AppStateWithCounter {
    counter: Mutex<i32>,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/index")]
async fn get_count(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    HttpResponse::Ok().body(format!("{counter}\n")) // <- response with count
}

#[post("/index")]
async fn add_count(data: web::Data<AppStateWithCounter>) -> impl Responder {
    let mut counter = data.counter.lock().unwrap(); 
    *counter += 1; 

    HttpResponse::Ok().body(format!("{counter}\n")) 
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("{}", "iu");
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
        // Note: web::Data created _outside_ HttpServer::new closure

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone()) 
            .service(hello)
            .service(echo)
            .service(add_count)
            .service(get_count)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}