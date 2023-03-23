mod logging;
mod models;

use actix_web::{web, App, HttpServer, Responder, get, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use slog::{Drain, info};

use models::Country;
use logging::SlogLogger;

#[derive(Clone)]
struct AppState {
    logger: slog::Logger,
    countries: Vec<Country>,
}

#[get("/v3.1/all")]
async fn get_countries(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&state.countries)
}

#[derive(Debug, Deserialize)]
pub struct GetCountryByNameParams {
    #[serde(rename = "fullText")]
    full_text: Option<bool>
}

#[get("/v3.1/name/{name}")]
async fn get_country_by_name(state: web::Data<AppState>, name: web::Path<String>, query: web::Query<GetCountryByNameParams>) -> impl Responder {
    let full_text_search = query.full_text.unwrap_or(false);
    let name = name.into_inner();
    let countries: Vec<&Country>; 
    if full_text_search {
        countries = state.countries.iter().filter(|c|
            c.name.common.eq_ignore_ascii_case(name.as_str()) ||
            c.name.official.eq_ignore_ascii_case(name.as_str())
        ).collect::<Vec<&Country>>();
    } else {
        countries = state.countries.iter().filter(|c|
            c.name.common.to_lowercase().contains(&name.to_lowercase()) ||
            c.name.official.to_lowercase().contains(&name.to_lowercase()) ||
            c.alt_spellings.as_ref().map(|a| 
                a.iter().any(|s| s.to_lowercase().contains(&name.to_lowercase()))
            ).unwrap_or(false) 
        ).collect::<Vec<&Country>>();
    }
    match countries.len() {
        0 => HttpResponse::NotFound().json(json!({
            "status": 404,
            "message": "Not Found"
        })),
        _ => HttpResponse::Ok().json(&countries)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let logger = slog::Logger::root(drain, slog::o!());

    info!(logger, "Bootstrapping server...");
    info!(logger, "Loading countries...");

    let json_bytes = include_bytes!("countries.json");
    let json_str = std::str::from_utf8(json_bytes).expect("Failed to convert JSON bytes to str");
    let countries: Vec<Country> = serde_json::from_str(json_str)?;

    info!(logger, "Countries loaded.");
    info!(logger, "Starting server on port 8080...");

    // App State
    let app_state = AppState {
        logger: logger,
        countries: countries,
    };

    // Start server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(SlogLogger)
            .service(get_countries)
            .service(get_country_by_name)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}