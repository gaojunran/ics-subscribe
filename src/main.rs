mod calendar;
mod config;

use axum::response::IntoResponse;
use axum::{Router, routing::get};
use config::Config;

async fn serve_ics() -> impl IntoResponse {
    let config = Config::load("config.toml").expect("failed to load config");
    let ics_content = calendar::build_calendar(&config);

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
    let app = Router::new().route("/calendar.ics", get(serve_ics));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind to port 3000");
    println!("Serving ICS at http://localhost:3000/calendar.ics");
    axum::serve(listener, app).await.unwrap();
}
