use crate::components::users::prelude::*;
use crate::prelude::*;

mod components;
mod shared;
pub mod prelude;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongo_uri = std::env::var("MONGODB_URI").expect("Missing MONGODB_URI");

    let client = Client::with_uri_str(mongo_uri).await.expect("failed to connect to MongoDB");
    setup_mongo(&client).await;

    println!("Server is up.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(create_user)
            .service(get_users)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
