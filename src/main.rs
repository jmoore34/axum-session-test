use std::net::SocketAddr;
use axum_database_sessions::{AxumSession, AxumSessionConfig, AxumSessionStore, axum_session_runner};
use axum::{
    Router,
    routing::get,
};

#[tokio::main]
async fn main() {
    let session_config = AxumSessionConfig::default()
        .with_table_name("test_table");

    let session_store = AxumSessionStore::new(None, session_config);

    // build our application with some routes
    let app = Router::new()
        .route("/greet/:name", get(greet))
        .layer(tower_cookies::CookieManagerLayer::new())
        .layer(axum_session_runner!(session_store));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet(session: AxumSession) -> String {
    let mut count: usize = session.get("count").await.unwrap_or(0);
    count += 1;
    session.set("count", count).await;

    count.to_string()
}