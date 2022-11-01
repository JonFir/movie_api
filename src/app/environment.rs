use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Environment {
    pub url: String,
    pub port: u16,
    pub database_url: String,
    pub database_max_connections: u32,
    pub posters_path: String,
    pub upload_file_size_limit: u32,
}

impl Environment {
    pub fn load() -> Result<Environment, envy::Error> {
        dotenv::dotenv().map_err(|_| envy::Error::Custom("missing .env file".to_owned()))?;
        envy::from_env::<Environment>()
    }

    pub fn socket_addrs(&self) -> (String, u16) {
        (self.url.to_owned(), self.port)
    }
}
