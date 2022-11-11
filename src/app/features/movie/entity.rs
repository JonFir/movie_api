use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::app::database;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: Option<i64>,
    pub title: String,
    pub director: String,
    pub relise_date: i64,
    pub rating: i64,
    pub poster_id: String,
    pub created_at: Option<NaiveDateTime>,
}

impl From<database::entity::Movie> for Movie {
    fn from(movie: database::entity::Movie) -> Self {
        Movie {
            id: movie.id.into(),
            title: movie.title,
            director: movie.director,
            relise_date: movie.relise_date,
            rating: movie.rating,
            poster_id: movie.poster_id,
            created_at: movie.created_at.into(),
        }
    }
}

impl Into<database::entity::Movie> for Movie {
    fn into(self) -> database::entity::Movie {
        database::entity::Movie {
            id: self.id.unwrap_or(0),
            title: self.title,
            director: self.director,
            relise_date: self.relise_date,
            rating: self.rating,
            poster_id: self.poster_id,
            created_at: self.created_at.unwrap_or(NaiveDateTime::default()),
        }
    }
}
