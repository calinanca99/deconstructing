use std::sync::{Arc, Mutex};

use models::{RegisteredUsers, Sessions, Users};

pub mod handlers;
pub mod models;

#[derive(Clone)]
pub struct AppState {
    pub registered_users: Arc<Mutex<RegisteredUsers>>,
    pub sessions: Arc<Mutex<Sessions>>,
    pub users: Arc<Mutex<Users>>,
}
