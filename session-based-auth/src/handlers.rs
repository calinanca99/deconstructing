use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
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

/// 1. Verify if the request contains an "authorization" header
/// 2. Extract the session id and see if it's associated with an user
/// 3. Return user's data
pub async fn home(
    State(state): State<AppState>,
    authorization: Option<TypedHeader<Authorization<Bearer>>>,
) -> String {
    match authorization {
        None => {
            "Not logged in. Log in by going to /login or create an account at /signup".to_string()
        }
        Some(bearer_token) => {
            let session = bearer_token.token().to_string();
            let sessions = state.sessions.lock().unwrap();

            if !sessions.contains_key(&session) {
                return "Session is invalid".to_string();
            }

            let username = sessions.get(&session).unwrap().clone();
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
pub async fn login(State(state): State<AppState>, Json(payload): Json<LoginDetails>) -> String {
    let registered_users = state.registered_users.lock().unwrap();

    match registered_users.get(&payload.username) {
        Some(password) => {
            if *password != payload.password {
                return "Username and password combination is wrong".to_string();
            }

            let mut sessions = state.sessions.lock().unwrap();
            let session = calculate_hash(&payload.username).to_string();

            if sessions.contains_key(&session) {
                return session;
            }

            sessions.insert(session.clone(), payload.username);

            session
        }
        None => "Username and password combination is wrong".to_string(),
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
