pub mod config;
use config::Config;

use axum::{routing::get, Router};
use tokio::signal;
use tracing::info;

async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn run(config: Config) {
    let app = Router::new().route("/", get(hello_world));

    let socket = format!("0.0.0.0:{}", &config.port)
        .parse()
        .expect("Invalid socket");

    info!("listening on {}", &socket);
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
