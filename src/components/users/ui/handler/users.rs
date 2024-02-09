use crate::components::users::prelude::*;
use crate::prelude::*;

#[get("/users")]
pub async fn get_users(mongo_client: web::Data<Client>) -> impl Responder {
    get_all_users_case(mongo_client).await
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .and_then(|users| {
            let user_list: Vec<UserDto> = users.into_iter().map(UserDto::from_domain).collect();
            serde_json::to_string(&user_list)
                .map_err(|_| HttpResponse::InternalServerError().finish())
                .map(|serialized| HttpResponse::Ok().body(serialized))
        })
        .unwrap_or_else(|response| response)
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(mongo_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let user_id = UserIdDto::from_string(path.into_inner());

    if let Some(user_id_dto) = user_id {
        get_user_by_id_case(mongo_client, user_id_dto).await
            .map_err(|err| match err {
                DatabaseError::UserNotFound(_) => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish()
            })
            .and_then(|user| user_to_dto(user))
            .unwrap_or_else(|response| response)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[post("/users")]
pub async fn create_user(mongo_client: web::Data<Client>, dto: web::Json<CreateUserDto>) -> impl Responder {
    create_user_case(mongo_client, &dto.username).await
        .map_err(|err| match err {
            DatabaseError::Write(_) => HttpResponse::BadRequest().finish(),
            _ => HttpResponse::InternalServerError().finish()
        })
        .and_then(|user| user_to_dto(user))
        .unwrap_or_else(|response| response)
}

#[delete("/users/{user_id}")]
pub async fn delete_user(mongo_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let user_id = UserIdDto::from_string(path.into_inner());

    if let Some(user_id_dto) = user_id {
        delete_user_case(mongo_client, user_id_dto).await
            .map_err(|err| match err {
                DatabaseError::UserNotFound(_) => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish()
            })
            .and_then(|_| Ok(HttpResponse::Ok().finish()))
            .unwrap_or_else(|response| response)
    } else {
        HttpResponse::BadRequest().finish()
    }
}


#[put("/users/{user_id}")]
pub async fn update_user(mongo_client: web::Data<Client>, path: web::Path<String>, dto: web::Json<CreateUserDto>) -> impl Responder {
    let user_id = UserIdDto::from_string(path.into_inner());

    if let Some(user_id_dto) = user_id {
        update_user_case(mongo_client, user_id_dto, &dto.username).await
            .map_err(|err| match err {
                DatabaseError::UserNotFound(_) => HttpResponse::NotFound().finish(),
                DatabaseError::Write(_) => HttpResponse::BadRequest().finish(),
                DatabaseError::Update(_) => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish()
            })
            .and_then(|user| user_to_dto(user))
            .unwrap_or_else(|response| response)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

fn user_to_dto(user: User) -> Result<HttpResponse, HttpResponse> {
    serde_json::to_string(&UserDto::from_domain(user))
        .map_err(|_| HttpResponse::InternalServerError().finish())
        .and_then(|serialized| Ok(HttpResponse::Ok().body(serialized)))
}
