
use actix_web:: {get, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize};


#[derive(Serialize)]
struct Entity {
    id: String,
    name: String,
    entity_type: String, 
    notes: String, 
}


#[get("/health")]
async fn health() -> impl Responder {
    "ok"
}


#[get("/entities")]
async fn list_entities() -> HttpResponse {
    let entities = vec![
        Entity {
            id: "1".to_string(),
            name: "Alice Example".to_string(),
            entity_type: "person".to_string(),
            notes: "Test entity".to_string(),
        },
        Entity {
            id: "2".to_string(),
            name: "Apple Co".to_string(),
            entity_type: "org".to_string(),
            notes: "Test entity 2".to_string(),
        }
    ];

    HttpResponse::Ok().json(entities)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new( || {
        App::new()
            .service(health)
            .service(list_entities)
        })
        .bind(("0.0.0.0", 8081))?
        .run()
        .await

}