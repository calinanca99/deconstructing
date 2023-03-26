use axum::{
    extract::State,
    headers::Cookie,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, TypedHeader,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    models::{CreateAccount, LoginDetails, User},
    AppState,
};

/// 1. Verify if the request contains cookies
/// 2. Verify if the cookies contains the "sid" cookie
/// 3. Extract the session id from the "sid" cookie and see if it's associated with an user
/// 4. Return user's data
pub async fn home(State(state): State<AppState>, cookies: Option<TypedHeader<Cookie>>) -> String {
    match cookies {
        None => {
            "Not logged in. Log in by going to /login or create an account at /signup".to_string()
        }
        Some(cookies) => {
            if cookies.get("sid").is_none() {
                return "Not logged in. Log in by going to /login or create an account at /signup"
                    .to_string();
            }

            let session = cookies.get("sid").unwrap();
            let sessions = state.sessions.lock().unwrap();

            if !sessions.contains_key(session) {
                return "Session is invalid. Please log in by going to /login".to_string();
            }

            let username = sessions.get(session).unwrap().clone();
            let users = state.users.lock().unwrap();

            let viewer = users.get(&username).expect("Unexpected failure");

            format!("Hello, {}!", viewer.username)
        }
    }
}

pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<CreateAccount>,
) -> &'static str {
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

/// 1. Check if username and password combination is correct
/// 2. Retrieve or create a new session session for the user
/// 3. Return the session session id
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginDetails>) -> Response {
    let registered_users = state.registered_users.lock().unwrap();

    let error_response = Response::builder()
        .status(StatusCode::UNAUTHORIZED)
        .header("Content-Type", "ext/plain; charset=utf-8")
        .body("Username and password combination is wrong".to_string())
        .unwrap();

    match registered_users.get(&payload.username) {
        Some(password) => {
            if *password != payload.password {
                return error_response.into_response();
            }

            let mut sessions = state.sessions.lock().unwrap();
            let session = calculate_hash(&payload.username).to_string();

            let session_cookie = format!("sid={}", session);

            let ok_response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "ext/plain; charset=utf-8")
                .header("set-cookie", session_cookie)
                .body("You're logged in".to_string())
                .unwrap();

            if sessions.contains_key(&session) {
                return ok_response.into_response();
            }

            sessions.insert(session.clone(), payload.username);

            return ok_response.into_response();
        }
        None => error_response.into_response(),
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
