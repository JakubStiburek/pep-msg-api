use crate::components::users::prelude::*;
use crate::prelude::*;

pub async fn delete_user_case(mongo_client: web::Data<Client>, id: UserIdDto) -> Result<(), DatabaseError> {
    let repository = UserRepository::new(&mongo_client);

    repository.delete(id.to_domain()).await
}
