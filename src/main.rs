use actix_cors::Cors;
use actix_web::{get, middleware, web, App, HttpServer, Responder};
use serde::Serialize;


// ----------------------------
// Configuración
// ----------------------------
const PORT: u16 = 4444;

// ----------------------------
// Handlers
// ----------------------------
#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/")]
async fn index() -> impl Responder {
    web::Json(Message {
        message: "Backend en Rust funcionando 🦀".into(),
    })
}

// ----------------------------
// Main
// ----------------------------
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor corriendo en http://localhost:{}", PORT);

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind(("0.0.0.0", PORT))?
    .run()
    .await
}
