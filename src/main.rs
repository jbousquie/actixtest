use actix_web::{post, web, App, HttpResponse, HttpServer};
use actix_files::Files;
use serde::Deserialize;


#[derive(Deserialize)]
struct FormData {
    username: String,
    password: String,
}

#[post("/login")] 
async fn login(form: web::Form<FormData>) -> HttpResponse  {
    HttpResponse::Ok().body(format!("username: {}, password : {}", form.username, form.password))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(login)
        .service(Files::new("/static", "./static/")
        ))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
