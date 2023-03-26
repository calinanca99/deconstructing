use std::sync::{Arc, Mutex};

use models::{RegisteredUsers, Tokens, Users};

pub mod handlers;
pub mod models;

#[derive(Clone)]
pub struct AppState {
    pub registered_users: Arc<Mutex<RegisteredUsers>>,
    pub tokens: Arc<Mutex<Tokens>>,
    pub users: Arc<Mutex<Users>>,
}
