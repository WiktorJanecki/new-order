use axum::{middleware, routing::get_service, Router};
use backend::{middlewares::mw_tracing, routes, AppState};
use jwt_simple::prelude::HS256Key;
use sqlx::postgres::PgPoolOptions;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    // print tracing with .env configuration
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_target(false),
        )
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // setup database
    let db_address = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().connect(&db_address).await?;

    // setup jwt
    let key = HS256Key::generate();

    let state = AppState {
        db: pool,
        jwt_key: key,
    };

    let app = Router::new()
        .nest("/api", routes::routes())
        .nest_service("/", get_service(ServeDir::new("./dist")))
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new())
        .layer(CorsLayer::very_permissive())
        .layer(middleware::map_response(mw_tracing)) // new line for each request
        .with_state(state);

    let address = std::env::var("SERVER_FULL_ADDRESS")?;
    let listener = tokio::net::TcpListener::bind(address).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
