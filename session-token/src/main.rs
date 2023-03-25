#![allow(dead_code)]

use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    sync::{Arc, Mutex},
};

use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    routing::{get, post},
    Json, Router, TypedHeader,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccount {
    username: Username,
    password: Password,
    bio: String,
    location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginDetails {
    username: Username,
    password: Password,
}

#[derive(Debug)]
pub struct User {
    username: Username,
    bio: String,
    location: Option<String>,
}

type Token = String;
type Username = String;
type Password = String;

type RegisteredUsers = HashMap<Username, Password>;
type Tokens = HashMap<Token, Username>;
type Users = HashMap<Username, User>;

#[derive(Clone)]
struct AppState {
    registered_users: Arc<Mutex<RegisteredUsers>>,
    tokens: Arc<Mutex<Tokens>>,
    users: Arc<Mutex<Users>>,
}

/// Handler for GET http://localhost:4000/home
///
/// 1. Verify if the request contains an "authorization" header
/// 2. Extract the session token and see if the server recognizes it
/// 3. Return user-specific info based on the token
async fn home(
    State(state): State<AppState>,
    // Can `authorization` be optional?
    authorization: Option<TypedHeader<Authorization<Bearer>>>,
) -> String {
    if authorization.is_none() {
        return "Not logged in. Log in by going to /login or create an account at /signup"
            .to_string();
    }

    let token = authorization.unwrap().token().to_string();
    let tokens = state.tokens.lock().unwrap();

    if !tokens.contains_key(&token) {
        return "Token is invalid".to_string();
    }

    let username = tokens.get(&token).unwrap().clone();
    let users = state.users.lock().unwrap();

    let viewer = users.get(&username).expect("Unexpected failure");

    return format!("Hello, {}!", viewer.username);
}

/// Handle for POST http://localhost:4000/singup
async fn signup(State(state): State<AppState>, Json(payload): Json<CreateAccount>) -> &'static str {
    let mut registered_users = state.registered_users.lock().unwrap();

    match registered_users.entry(payload.username.clone()) {
        std::collections::hash_map::Entry::Occupied(_) => "Username is taken!",
        std::collections::hash_map::Entry::Vacant(entry) => {
            let user = User {
                username: payload.username.clone(),
                bio: payload.bio,
                location: payload.location,
            };

            entry.insert(payload.password);

            let mut users = state.users.lock().unwrap();
            users.insert(payload.username, user);

            "Account created successfully"
        }
    }
}

/// Handle for POST http://localhost:4000/login
///
/// 1. Check if username and password combination is correct
/// 2. Retrieve or create a new session token for the user
/// 3. Return the session token
async fn login(State(state): State<AppState>, Json(payload): Json<LoginDetails>) -> String {
    let registered_users = state.registered_users.lock().unwrap();

    match registered_users.get(&payload.username) {
        Some(password) => {
            // TODO: Refactor this to have early returns
            if *password == payload.password {
                let mut tokens = state.tokens.lock().unwrap();
                let token = calculate_hash(&payload.username).to_string();

                if tokens.contains_key(&token) {
                    return token.clone();
                }

                tokens.insert(token.clone(), payload.username);

                token
            } else {
                "Username and password combination is wrong".to_string()
            }
        }
        None => "Username and password combination is wrong".to_string(),
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

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
