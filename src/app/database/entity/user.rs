#[derive(sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub hash: String,
    pub email: String,
}
