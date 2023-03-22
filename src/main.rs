mod models;

use actix_web::{web, App, HttpServer, Responder, get, HttpResponse};
use actix_web::dev::Service;
use slog::{Drain, info, warn};

use models::Country;

#[derive(Clone)]
struct AppState {
    logger: slog::Logger,
    countries: Vec<Country>,
}

#[get("/v3.1/all")]
async fn get_countries(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&state.countries)
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
            .wrap_fn(|req, srv| {
                let logger = req.app_data::<web::Data<AppState>>().unwrap().logger.clone();
                let req_method = req.method().to_string();
                let req_path = req.path().to_string();
                let start = std::time::Instant::now();
                let fut = srv.call(req);
                async move {
                    let res = fut.await;
                    let elapsed = start.elapsed();
                    let status = res.as_ref().map(|r| r.status());
                    if let Ok(status) = status {
                        if status.is_client_error() || status.is_server_error() {
                            warn!(logger, "{} {} {} {:?}", req_method, req_path, status, elapsed);
                        } else {
                            info!(logger, "{} {} {} {:?}", req_method, req_path, status, elapsed);
                        }
                    }
                    res
                }
            })
            .service(get_countries)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}