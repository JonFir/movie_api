use uuid::Uuid;

use crate::app::errors::Error;

use super::{db::DB, entity::User};

impl DB {
    pub async fn create_user(
        &self,
        username: &str,
        hash: &str,
        email: &str,
    ) -> Result<User, Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query!(
            "
        INSERT INTO users (id, username, hash, email) 
        VALUES ($1, $2, $3, $4)",
            id,
            username,
            hash,
            email,
        )
        .execute(&self.pool)
        .await?;

        let user = User {
            id,
            username: username.to_owned(),
            hash: hash.to_owned(),
            email: email.to_owned(),
        };
        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT *
            FROM users
            WHERE username = $1",
            username,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn find_by_token(&self, token: &str) -> Result<Option<User>, Error> {
        let user = sqlx::query_as!(
            User,
            "
            SELECT u.id, u.username, u.email, u.hash
            FROM users as u 
            JOIN remember_tokens as rt ON u.id = rt.user_id
            WHERE rt.token = $1",
            token,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    pub async fn set_remember_tokens(&self, id: &str, token: &str) -> Result<(), Error> {
        sqlx::query!(
            "
        INSERT INTO remember_tokens (user_id, token) 
        VALUES ($1, $2)",
            id,
            token,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
