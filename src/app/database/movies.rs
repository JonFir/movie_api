use chrono::{DateTime, Utc};

use super::{db::DB, entity::Movie, errors::Error};
impl DB {
    pub async fn create_movie(
        &self,
        title: String,
        director: String,
        relise_date: i16,
        rating: i8,
        poster_id: String,
    ) -> Result<Movie, Error> {
        let created_at = chrono::offset::Utc::now();
        let result = sqlx::query!(
            "
        INSERT INTO movies (title, director, relise_date, rating, poster_id, created_at) 
        VALUES ($1, $2, $3, $4, $5, $6)",
            title,
            director,
            relise_date,
            rating,
            poster_id,
            created_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|error| {
            let e = error.as_database_error().and_then(|e| e.code());
            match e {
                Some(err) if err.eq("2067") => Error::AlreadyExist(error),
                _ => Error::Other(error),
            }
        })?;
        let id = result.last_insert_rowid();

        let user = Movie {
            id,
            title,
            director,
            relise_date,
            rating,
            poster_id,
            created_at,
        };
        Ok(user)
    }

    pub async fn find_movie_by_id(&self, id: i64) -> Result<Option<Movie>, Error> {
        let movie = sqlx::query_as_unchecked!(
            Movie,
            "SELECT *
            FROM movies as m
            WHERE id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::NotFound(e))?;
        Ok(movie)
    }
    pub async fn all_movies(&self, cursor: i64, count: i64) -> Result<Vec<Movie>, Error> {
        let movie = sqlx::query_as_unchecked!(
            Movie,
            "SELECT *
            FROM movies as m
            WHERE id < $1
            LIMIT $2",
            cursor,
            count,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Error::NotFound(e))?;
        Ok(movie)
    }
    pub async fn delete_movie(&self, id: i64) -> Result<(), Error> {
        let result = sqlx::query!(
            "DELETE FROM movies
            WHERE id = $1",
            id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::NotFound(e))?;
        Ok(())
    }
}