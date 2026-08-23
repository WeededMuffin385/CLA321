use std::ops::{AddAssign, SubAssign};
use axum::extract::State;
use axum::Router;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use tracing::info;
use crate::context::Context;

pub fn router() -> Router<Context> {
    Router::new()
        .route("/", get(handler))
        .route("/add", get(add))
        .route("/sub", get(sub))

}

async fn handler(
    State(state): State<Context>,
) -> Response {
    Html(format!("
        <h1>Hello, World!</h1>
        <h2>This response was constructed in the hello-world-service</h2>
        <h3>The current value of the global counter is {}</h3>
    ", state.0.counter.lock().unwrap())).into_response()
}

async fn add(
    State(state): State<Context>,
) -> Response {
    state.0.counter.lock().unwrap().add_assign(1);

    info!("counter increased by 1. the new value is {}", state.0.counter.lock().unwrap());

    Html(format!("
        <h1>The global counter value was increased by 1</h1>
        <h2>The current value of the global counter is {}</h2>
    ", state.0.counter.lock().unwrap())).into_response()
}

async fn sub(
    State(state): State<Context>,
) -> Response {
    state.0.counter.lock().unwrap().sub_assign(1);

    info!("counter decreased by 1. the new value is {}", state.0.counter.lock().unwrap());

    Html(format!("
        <h1>The global counter value was subtracted by 1</h1>
        <h2>The current value of the global counter is {}</h2>
    ", state.0.counter.lock().unwrap())).into_response()
}