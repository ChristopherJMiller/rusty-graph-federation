use anyhow::Result;
use async_graphql::{
    extensions::Tracing, http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_poem::*;
use poem::{listener::TcpListener, web::Html, *};
use query::Query;
use tracing::info;

mod query;
mod user;

#[handler]
async fn graphiql() -> impl IntoResponse {
    Html(GraphiQLSource::build().finish())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // create the schema
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .enable_federation()
        .extension(Tracing)
        .finish();

    // start the http server
    let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
    info!("GraphiQL: http://0.0.0.0:8000");
    Server::new(TcpListener::bind("0.0.0.0:8000"))
        .run(app)
        .await?;
    Ok(())
}
