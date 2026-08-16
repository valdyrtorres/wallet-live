use axum::Router;

use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{
    Layer, fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt
};

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    async fn new() -> color_eyre::Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = std::env::var("DATABASE_URL")?;
        let db = PgPool::connect(&database_url).await?;
        sqlx::migrate!("./migrations").run(&db).await?;

        Ok(Self { db })
    }
}

pub struct App;

impl App {
    pub async fn start() -> color_eyre::Result<()> {
        let layer = tracing_subscriber::fmt::layer()
            .with_span_events(FmtSpan::NEW)
            .boxed();

        tracing_subscriber::registry().with(layer).init();

        let state = AppState::new().await?;

        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        let router = Router::new()  
            .nest("/api", routes::api::router())
            .with_state(state);

        info!("Server listening on http://0.0.0.0:3000");

        axum::serve(listener, router).await?;

        Ok(())
    }
}
