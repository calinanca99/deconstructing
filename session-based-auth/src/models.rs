use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateAccount {
    pub username: Username,
    pub password: Password,
    pub bio: String,
    pub location: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginDetails {
    pub username: Username,
    pub password: Password,
}

#[derive(Debug)]
pub struct User {
    pub username: Username,
    pub bio: String,
    pub location: Option<String>,
}

pub type Session = String;
pub type Username = String;
pub type Password = String;

pub type RegisteredUsers = HashMap<Username, Password>;
pub type Sessions = HashMap<Session, Username>;
pub type Users = HashMap<Username, User>;
