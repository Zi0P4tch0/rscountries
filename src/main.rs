mod models;

use actix_web::{web, App, HttpServer, Responder, get, HttpResponse};

use models::Country;

#[get("/v3.1/all")]
async fn get_countries(countries: web::Data<Vec<Country>>) -> impl Responder {
    HttpResponse::Ok().json(countries)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Load countries
    println!("Loading countries...");
    let json_bytes = include_bytes!("../countries.json");
    let json_str = std::str::from_utf8(json_bytes).expect("Failed to convert JSON bytes to str");
    let countries: Vec<Country> = serde_json::from_str(json_str)?;

    // Start server
    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(countries.clone()))
            .service(get_countries)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}