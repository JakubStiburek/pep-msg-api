use crate::components::users::prelude::*;
use crate::prelude::*;

pub async fn create_user_case(mongo_client: web::Data<Client>, username: impl ToString) -> Result<User, InfrastructureError> {
    let repository = UserRepository::new(&mongo_client);

    repository.create(Username::new(username)).await
}
