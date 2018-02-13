use rocket::response::{Responder, Response};
use rocket::request::Request;
use rocket::http::Status;
use rocket_contrib::Json;
use rocket_cors::{self, Guard};

use app::end_user::{DetailedEndUserRepr, EndUserRepr, EndUserService, EndUserServiceComponent,
                    RegisterEndUserCmd, UpdateEndUserCmd};
use domain::error::domain as ed;
use constant;
use infra::rest::common::{AuthorizationHeader, AuthorizationType, CommonListResponse,
                          CommonResponse};
use infra::session::RedisStore;
use util::{convert_str_to_naive_date, generate_random_id};
use server::Server;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserLoginForm {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserLoginResponse {
    pub sid: String,
    pub end_user_id: String,
}

impl<'r> Responder<'r> for EndUserLoginResponse {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

#[post("/login", data = "<input>")]
pub fn login(
    input: Json<EndUserLoginForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<EndUserLoginResponse, ed::Error> {
    let form = input.into_inner();
    let name = form.name;
    let password = form.password;
    let service = server.end_user_service();
    let ret = service.log_in(&name, &password)?;
    let mut sid = generate_random_id(64usize);
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            sid = token.clone();
            redis_store.set(&sid, constant::END_USER_SESS_ID_FIELD, &ret.id)?;
        } else {
            redis_store.set(&sid, constant::END_USER_SESS_ID_FIELD, &ret.id)?;
        }
    } else {
        redis_store.set(&sid, constant::END_USER_SESS_ID_FIELD, &ret.id)?;
    }
    Ok(EndUserLoginResponse {
        sid,
        end_user_id: ret.id.clone(),
    })
}

#[post("/logout")]
pub fn logout(
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
) -> Result<(), ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            redis_store.del(&token, None)?
        }
    }
    Ok(())
}

#[get("/")]
pub fn get_end_users<'r>(
    cors: Guard<'r>,
    server: Server,
) -> rocket_cors::Responder<Result<CommonListResponse<EndUserRepr>, ed::Error>> {
    let service = server.end_user_service();
    cors.responder(
        service
            .get_end_users()
            .map(|v| CommonListResponse { list: v }),
    )
}

#[get("/<id>")]
pub fn get_end_user<'r>(
    cors: Guard<'r>,
    id: String,
    server: Server,
) -> rocket_cors::Responder<Result<EndUserRepr, ed::Error>> {
    let service = server.end_user_service();
    cors.responder(service.get_end_user(&id))
}

#[get("/<id>/detail")]
pub fn get_detailed_end_user(
    id: String,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<DetailedEndUserRepr, ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let end_user_self_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let service = server.end_user_service();
            return service.get_detailed_end_user(&id, &end_user_self_id, &admin_id);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserRegisterForm {
    pub name: String,
    pub password: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub nickname: Option<String>,
    pub profile: Option<String>,
    pub picture: Option<String>,
    pub website: Option<String>,
    pub gender: Option<String>,
    pub birthdate: Option<String>,
    pub zoneinfo: Option<String>,
    pub locale: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserRegisterResponse {
    pub end_user_id: String,
}

impl<'r> Responder<'r> for EndUserRegisterResponse {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

#[post("/", data = "<input>")]
pub fn register_end_user<'r>(
    cors: Guard<'r>,
    input: Json<EndUserRegisterForm>,
    server: Server,
) -> rocket_cors::Responder<Result<EndUserRegisterResponse, ed::Error>> {
    let form = input.into_inner();
    // Parse ISO8601 timestamp e.g. 2000-1-1T00:00:00.000+09:00
    let birthdate = match form.birthdate
        .map_or(Ok(None), |d| convert_str_to_naive_date(&d).map(|v| Some(v)))
    {
        Ok(v) => v,
        Err(e) => return cors.responder(Err(e)),
    };
    let cmd = RegisterEndUserCmd {
        name: form.name,
        password: form.password,
        email: form.email,
        given_name: form.given_name,
        family_name: form.family_name,
        middle_name: form.middle_name,
        nickname: form.nickname,
        profile: form.profile,
        picture: form.picture,
        website: form.website,
        gender: form.gender,
        birthdate,
        zoneinfo: form.zoneinfo,
        locale: form.locale,
        phone_number: form.phone_number,
    };
    let service = server.end_user_service();
    cors.responder(
        service
            .register_end_user(&cmd)
            .map(|r| EndUserRegisterResponse { end_user_id: r.id }),
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserUpdateForm {
    pub name: String,
    pub new_password: Option<String>,
    pub current_password: Option<String>,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub nickname: Option<String>,
    pub profile: Option<String>,
    pub picture: Option<String>,
    pub website: Option<String>,
    pub gender: Option<String>,
    pub birthdate: Option<String>,
    pub zoneinfo: Option<String>,
    pub locale: Option<String>,
    pub phone_number: Option<String>,
}

#[put("/<id>", data = "<input>")]
pub fn update_end_user(
    id: String,
    input: Json<EndUserUpdateForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<(), ed::Error> {
    let form = input.into_inner();
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let end_user_self_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let cmd = UpdateEndUserCmd {
                target_id: id,
                self_id: end_user_self_id,
                admin_id,
                name: form.name,
                email: form.email,
                new_password: form.new_password,
                current_password: form.current_password,
                given_name: form.given_name,
                family_name: form.family_name,
                middle_name: form.middle_name,
                nickname: form.nickname,
                profile: form.profile,
                picture: form.picture,
                website: form.website,
                gender: form.gender,
                birthdate: form.birthdate
                    .map_or(Ok(None), |d| convert_str_to_naive_date(&d).map(|v| Some(v)))?,
                zoneinfo: form.zoneinfo,
                locale: form.locale,
                phone_number: form.phone_number,
            };
            let service = server.end_user_service();
            return service.update_end_user(&cmd);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

#[delete("/<id>")]
pub fn delete_end_user(
    id: String,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<(), ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let end_user_self_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let service = server.end_user_service();
            return service.delete_end_user(&id, &end_user_self_id, &admin_id);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}
