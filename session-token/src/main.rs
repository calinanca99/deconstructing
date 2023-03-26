use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Router,
};
use session_token::{
    handlers::{home, login, signup},
    AppState,
};

#[tokio::main]
async fn main() {
    let state = AppState {
        registered_users: Arc::new(Mutex::new(HashMap::new())),
        tokens: Arc::new(Mutex::new(HashMap::new())),
        users: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/home", get(home))
        .route("/login", post(login))
        .route("/signup", post(signup))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap()
}
