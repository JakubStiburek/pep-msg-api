use crate::components::users::prelude::*;
use crate::prelude::*;

#[get("/users")]
pub async fn get_users(mongo_client: web::Data<Client>) -> impl Responder {
    match get_all_users_case(mongo_client).await {
        Ok(users) => {
            let user_list: Vec<UserDto> = users.into_iter().map(|user| { UserDto::from_domain(user) }).collect();
            match serde_json::to_string(&user_list) {
                Ok(serialized) => HttpResponse::Ok().body(serialized),
                Err(_) => HttpResponse::InternalServerError().finish()
            }
        }
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}

#[get("/users/{user_id}")]
pub async fn get_user_by_id(mongo_client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let user_id = UserIdDto::from_string(path.into_inner());
    if let Some(dto) = user_id {
        match get_user_by_id_case(mongo_client, dto).await {
            Ok(res) => {
                let user_dto = UserDto::from_domain(res);
                match serde_json::to_string(&user_dto) {
                    Ok(serialized) => HttpResponse::Ok().body(serialized),
                    Err(_) => HttpResponse::InternalServerError().finish()
                }
            }
            Err(err) => {
                match err {
                    DatabaseError::UserNotFound(_) => HttpResponse::NotFound().finish(),
                    _ => HttpResponse::InternalServerError().finish()
                }
            }
        }
    } else {
        HttpResponse::BadRequest().finish()
    }
}

#[post("/users")]
pub async fn create_user(mongo_client: web::Data<Client>, dto: web::Json<CreateUserDto>) -> impl Responder {
    match create_user_case(mongo_client, &dto.username).await {
        Ok(res) => {
            let user_dto = UserDto::from_domain(res);
            match serde_json::to_string(&user_dto) {
                Ok(serialized) => HttpResponse::Ok().body(serialized),
                Err(_) => HttpResponse::InternalServerError().finish()
            }
        }
        Err(err) => {
            match err {
                DatabaseError::Write(_) => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
}

//#[delete("/users/{user_id}")]
//pub async fn delete_user_by_id(path: web::Path<String>) -> impl Responder {
//    let user_id = path.into_inner();
//    HttpResponse::Ok().body(user_id)
//}
//
//#[put("/users/{user_id}")]
//pub async fn update_user(path: web::Path<String>, dto: web::Json<UpdateUserDto>) -> impl Responder {
//    let user_id = path.into_inner();
//    HttpResponse::Ok().body(format!("id: {} username: {}", user_id, dto.username))
//}
