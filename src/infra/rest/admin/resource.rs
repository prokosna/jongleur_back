use rocket::response::{Responder, Response};
use rocket::request::Request;
use rocket::http::Status;
use rocket_contrib::Json;
use rocket_cors::{self, Guard};

use app::admin::{AdminRepr, AdminService, AdminServiceComponent, RegisterAdminCmd, UpdateAdminCmd};
use domain::error::domain as ed;
use domain::constant;
use infra::rest::common::{AuthorizationHeader, AuthorizationType, CommonResponse};
use infra::session::RedisStore;
use util::generate_random_id;
use server::Server;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminLoginForm {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminLoginResponse {
    pub sid: String,
    pub admin_id: String,
}

impl<'r> Responder<'r> for AdminLoginResponse {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

#[post("/login", data = "<input>")]
pub fn login(
    input: Json<AdminLoginForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<AdminLoginResponse, ed::Error> {
    let form = input.into_inner();
    let name = form.name;
    let password = form.password;
    let service = server.admin_service();
    let ret = service.log_in(&name, &password)?;
    let mut sid = generate_random_id(64usize);
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            sid = token.clone();
            redis_store.set(&sid, constant::ADMIN_SESS_ID_FIELD, &ret.id)?;
        } else {
            redis_store.set(&sid, constant::ADMIN_SESS_ID_FIELD, &ret.id)?;
        }
    } else {
        redis_store.set(&sid, constant::ADMIN_SESS_ID_FIELD, &ret.id)?;
    }
    Ok(AdminLoginResponse {
        sid,
        admin_id: ret.id.clone(),
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

#[get("/<id>")]
pub fn get_admin<'r>(
    id: String,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<AdminRepr, ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let self_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let service = server.admin_service();
            return service.get_admin(&id, &self_id);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminRegisterForm {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminRegisterResponse {
    pub admin_id: String,
}

impl<'r> Responder<'r> for AdminRegisterResponse {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

#[post("/", data = "<input>")]
pub fn register_admin<'r>(
    cors: Guard<'r>,
    input: Json<AdminRegisterForm>,
    server: Server,
) -> rocket_cors::Responder<Result<AdminRegisterResponse, ed::Error>> {
    let form = input.into_inner();
    let cmd = RegisterAdminCmd {
        name: form.name,
        password: form.password,
    };
    let service = server.admin_service();
    cors.responder(
        service
            .register_admin(&cmd)
            .map(|r| AdminRegisterResponse { admin_id: r.id }),
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminUpdateForm {
    pub name: String,
    pub new_password: Option<String>,
    pub current_password: String,
}

#[put("/<id>", data = "<input>")]
pub fn update_admin(
    id: String,
    input: Json<AdminUpdateForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<(), ed::Error> {
    let form = input.into_inner();
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let self_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let cmd = UpdateAdminCmd {
                target_id: id,
                self_id,
                name: form.name,
                new_password: form.new_password,
                current_password: form.current_password,
            };
            let service = server.admin_service();
            return service.update_admin(&cmd);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

#[delete("/<id>")]
pub fn delete_admin(
    id: String,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<(), ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let self_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let service = server.admin_service();
            return service.delete_admin(&id, &self_id);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}
