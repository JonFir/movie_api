use std::sync::Arc;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use super::{
    features::{self, auth::middleware::Middleware},
    state::AppState,
};

// #TODO: remove demo method
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn run(address: (String, u16), state: Arc<AppState>) -> Result<(), std::io::Error> {
    HttpServer::new(move || {
        let app_data = web::Data::new(Arc::clone(&state));

        let auth_scope = web::scope("/auth")
            .service(features::auth::register)
            .service(features::auth::login);

        let api_scope = web::scope("/api")
            .wrap(Middleware {
                app_state: Arc::clone(&state),
            })
            .service(hello);
        let image_scope = web::scope("/image")
            .wrap(Middleware {
                app_state: Arc::clone(&state),
            })
            .service(features::image::image)
            .service(features::image::upload);
        let movies_scope = web::scope("/movies")
            .wrap(Middleware {
                app_state: Arc::clone(&state),
            })
            .service(features::movie::create)
            .service(features::movie::movie)
            .service(features::movie::delete)
            .service(features::movie::list);

        // upload_handler
        let global_scope = hello;
        let upload_file_size_limit = 1024 * 1024 * state.environment.upload_file_size_limit;
        App::new()
            .app_data(app_data)
            .service(auth_scope)
            .service(api_scope)
            .service(image_scope)
            .service(movies_scope)
            .service(global_scope)
            .app_data(web::PayloadConfig::new(
                usize::try_from(upload_file_size_limit).unwrap(),
            ))
    })
    .bind(address)?
    .run()
    .await
}
