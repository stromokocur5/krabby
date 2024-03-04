use axum::{middleware, Router};
use deadpool_redis::{Config, Runtime};
use krabby::{AppState, Result};
use sqlx::PgPool;
use std::{env, sync::Arc};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "krabby=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let redis_cfg = Config::from_url(env::var("REDIS_URL")?);
    let redis = redis_cfg.create_pool(Some(Runtime::Tokio1))?;

    let pg_cfg = env::var("DATABASE_URL")?;
    let pg = PgPool::connect(&pg_cfg).await?;

    let state = AppState { pg, redis };
    let state = Arc::new(state);
    let app = Router::new()
        .merge(krabby::router())
        .with_state(state.clone())
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            krabby::routes::middleware::auth,
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    Ok(())
}
