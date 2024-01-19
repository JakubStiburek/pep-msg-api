use crate::components::users::prelude::*;
use crate::prelude::*;

pub async fn get_all_users_case(mongo_client: web::Data<Client>) -> Result<Vec<User>, InfrastructureError> {
    let repository = UserRepository::new(&mongo_client);

    repository.get_all().await
}
