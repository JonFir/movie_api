use app::{database, environment::Environment, server, state::AppState};
use std::{error::Error, sync::Arc};

mod app;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let environment = Environment::load()?;
    let address = environment.socket_addrs();
    let db = database::db::DB::new(
        &environment.database_url,
        environment.database_max_connections,
    )
    .await?;
    let state = AppState {
        database: db,
        environment,
    };
    let state = Arc::new(state);
    server::run(address, state).await?;
    Ok(())
}
