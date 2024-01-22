#[macro_use]
extern crate log;

use actix_web::middleware::Logger;
use env_logger::Env;

use crate::components::users::prelude::*;
use crate::prelude::*;

mod components;
mod shared;
pub mod prelude;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let mongo_uri = std::env::var("MONGODB_URI").expect("Missing MONGODB_URI");

    let client = Client::with_uri_str(mongo_uri).await.expect("failed to connect to MongoDB");
    setup_mongo(&client).await;

    info!("Server is up.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(create_user)
            .service(get_users)
            .service(get_user_by_id)
            .service(update_user)
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
