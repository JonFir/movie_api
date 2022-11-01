use serde::Serialize;

use crate::app::database;

#[derive(Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl From<database::entity::User> for User {
    fn from(user: database::entity::User) -> Self {
        User {
            id: user.id,
            username: user.username,
            email: user.email,
        }
    }
}
