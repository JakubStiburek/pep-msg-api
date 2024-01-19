use crate::components::users::infrastructure::mongo::user_repository::COLL_NAME;
use crate::components::users::prelude::UserModel;
use crate::prelude::*;

/// Creates an index on the "username" field to force the values to be unique.
async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<UserModel>(COLL_NAME)
        .create_index(model, None)
        .await
        .expect("creating an index should succeed");
}

pub async fn setup_mongo(client: &Client) {
    create_username_index(client).await
}
