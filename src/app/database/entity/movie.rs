use chrono::NaiveDateTime;

#[derive(sqlx::FromRow)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub director: String,
    pub relise_date: i64,
    pub rating: i64,
    pub poster_id: String,
    pub created_at: NaiveDateTime,
}
