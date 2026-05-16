mod calendar;
mod config;

use axum::response::IntoResponse;
use axum::{Router, routing::get};
use config::Config;

fn load_and_build(config_path: &str) -> String {
    let config = Config::load(config_path).expect("failed to load config");
    let ics_content = calendar::build_calendar(&config);

    if std::env::var("ICS_SAVE").is_ok() {
        let ics_path = config_path.trim_end_matches(".toml").to_string() + ".ics";
        std::fs::write(&ics_path, &ics_content).expect("failed to save ICS file");
        println!("Saved ICS to {ics_path}");
    }

    ics_content
}

async fn serve_ics() -> impl IntoResponse {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let ics_content = load_and_build(&config_path);

    (
        axum::http::StatusCode::OK,
        [(
            axum::http::header::CONTENT_TYPE,
            "text/calendar; charset=utf-8",
        )],
        ics_content,
    )
}

#[tokio::main]
async fn main() {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or_else(|_| "config.toml".to_string());
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);

    load_and_build(&config_path);

    let app = Router::new().route("/calendar.ics", get(serve_ics));
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind");
    println!("Serving ICS at http://localhost:{port}/calendar.ics (config: {config_path})");
    axum::serve(listener, app).await.unwrap();
}
