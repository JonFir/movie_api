use super::{database::db::DB, environment::Environment};

#[derive(Clone)]
pub struct AppState {
    pub database: DB,
    pub environment: Environment,
}
