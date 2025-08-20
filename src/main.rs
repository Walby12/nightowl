use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use actix_web::{get, App, HttpResponse, HttpRequest, HttpServer, Responder};

fn make_json(path: &str) {
    fs::File::create(path).unwrap();
}

fn append_json(path: &str, context: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();
    
    let json_to_write = format!("{context} {{\n\t{content}\n}}\n");

    let ms = file.write_all(json_to_write.as_bytes());
    match ms {
        Ok(_) => println!("Ok: succesfuly writed to file"),
        Err(e) => eprintln!("Error: Could not write to file, {}", e),
    }
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

async fn collect(req: &HttpRequest) -> impl Responder {
    if let Some(val) = req.peer_addr() {
        println!("Address {:?}", val.ip());
    };
    HttpResponse::Ok()
}

#[get("/")]
async fn home(req: HttpRequest) -> impl Responder {
    collect(&req).await;
    hello().await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_name = "messages.json";

    make_json(file_name);
    append_json(file_name, "walby", "1: hello\n\t2: bye\n");

    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
