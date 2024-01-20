use crate::components::users::prelude::*;
use crate::prelude::*;

pub async fn get_user_by_id_case(mongo_client: web::Data<Client>, id: UserIdDto) -> Result<User, DatabaseError> {
    let repository = UserRepository::new(&mongo_client);

    repository.get_by_id(id.to_domain()).await
}
