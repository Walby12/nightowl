use actix_web::{web, App, HttpResponse, HttpServer, Responder, post, get};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;
use actix_files::NamedFile;
use std::path::PathBuf;

#[derive(Serialize, Clone)]
struct Message {
    id: usize,
    user: String,
    text: String,
}

struct AppState {
    messages: Mutex<Vec<Message>>,
}

#[derive(Deserialize)]
struct JsonMessage {
    user: String,
    msg: String,
}

#[post("/send")]
async fn send_message(data: web::Data<AppState>, json: web::Json<JsonMessage>) -> impl Responder {
    let mut messages = data.messages.lock().unwrap();
    let id = messages.len();
    messages.push(Message { id, user: json.user.clone(), text: json.msg.clone() });
    HttpResponse::Ok().finish()
}

#[get("/recv")]
async fn recv_messages(data: web::Data<AppState>, query: web::Query<HashMap<String, String>>) -> impl Responder {
    let since = query.get("since").and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
    let messages = data.messages.lock().unwrap();
    let new_messages: Vec<Message> = messages.iter().filter(|m| m.id >= since).cloned().collect();
    HttpResponse::Ok().json(new_messages)
}

#[get("/")]
async fn home() -> actix_web::Result<NamedFile> {
    let path: PathBuf = "src/main.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        messages: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(send_message)
            .service(recv_messages)
            .service(home)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

