use axum::Router;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;

pub fn router() -> Router {
    Router::new().route("/", get(handler))
}

async fn handler() -> Response {
    Html("<h1>Hello, World!</h1>").into_response()
}
