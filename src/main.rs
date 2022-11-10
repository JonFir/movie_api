use log::error;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let result = course_api::run().await;
    if let Err(e) = result.as_ref() {
        error!("Fail run with: {:?}", e)
    }
    result
}
