use std::{
    future::Future,
    net::{SocketAddr, TcpListener},
};

use axum::{
    routing::{post, get},
    extract::Extension,
    AddExtensionLayer,
    response::{self, IntoResponse},
    Router};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::gql::schema::{QueryRoot, AppSchema};

async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

async fn healthcheck() -> &'static str {
    "OK"
}

async fn sdl(
    schema: Extension<AppSchema>,
) -> impl IntoResponse {
    schema.sdl()
}

pub async fn run(
    addr: impl Into<SocketAddr>,
    shutdown: impl Future<Output=()>,
) -> Result<(), anyhow::Error> {
    let addr = addr.into();
    let listener = std::net::TcpListener::bind(addr)?;

    info!(?addr, "listening");

    run_with_listener(listener, shutdown).await
}

async fn connect_db() -> Result<sea_orm::DatabaseConnection, anyhow::Error> {
    use sea_orm::{ConnectOptions,Database};

    // TODO: pass from args( or dotenv)
    let mut opt = ConnectOptions::new(std::env::var("DATABASE_URL").expect("environment variable DATABASE_URL required"));
    opt.max_connections(20).sqlx_logging(true);

    let db = Database::connect(opt).await?;

    Ok(db)
}

pub async fn run_with_listener(
    listener: TcpListener,
    shutdown: impl Future<Output=()>,
) -> Result<(), anyhow::Error> {
    let db_conn = connect_db().await?;

    let schema = AppSchema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(db_conn)
        .finish();

    let app = Router::new()
        .route("/graphql", post(graphql_handler))
        .route("/graphql/playground", get(graphql_playground))
        .route("/graphql/sdl", get(sdl))
        .route("/healthcheck",get(healthcheck))
        .layer(
            tower::ServiceBuilder::new()
                .layer(
                   TraceLayer::new_for_http()
                       .on_request(|request: &axum::http::Request<_>, _: &tracing::Span| {
                           tracing::info!(?request);
                       }),
                )
                .layer(AddExtensionLayer::new(schema))
        );

    axum::Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await
        .map_err(anyhow::Error::from)
}
