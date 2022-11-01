use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

#[derive(Clone)]
pub struct DB {
    pub(super) pool: Pool<Sqlite>,
}

impl DB {
    pub async fn new(uri: &str, max_connections: u32) -> Result<DB, sqlx::Error> {
        let pool = SqlitePoolOptions::new()
            .max_connections(max_connections)
            .connect(uri)
            .await?;
        let db = DB { pool };
        Ok(db)
    }
}
