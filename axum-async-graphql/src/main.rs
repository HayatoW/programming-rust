use async_graphql::{Schema, http::GraphiQLSource};
use axum::response::{self, IntoResponse};

mod starwars;

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

// #[tokio::main]
fn main() {
    let schema = Schema::build(query);
}
