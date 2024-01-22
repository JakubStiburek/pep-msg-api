use crate::components::users::prelude::*;
use crate::prelude::*;

pub async fn update_user_case(mongo_client: web::Data<Client>, id: UserIdDto, username: impl ToString) -> Result<User, DatabaseError> {
    let repository = UserRepository::new(&mongo_client);

    repository.update(id.to_domain(), Username::new(username)).await
}
