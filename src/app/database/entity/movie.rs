use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub director: String,
    pub relise_date: i16,
    pub rating: i8,
    pub poster_id: String,
    pub created_at: DateTime<Utc>,
}
