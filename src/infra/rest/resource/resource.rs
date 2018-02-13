use rocket::response::{Responder, Response};
use rocket::request::Request;
use rocket::http::Status;
use rocket_contrib::Json;
use rocket_cors::{self, Guard};

use app::resource::{DetailedResourceRepr, RegisterResourceCmd, ResourceRepr, ResourceService,
                    ResourceServiceComponent, UpdateResourceCmd};
use domain::error::domain as ed;
use constant;
use domain::model::Scope;
use infra::rest::common::{AuthorizationHeader, AuthorizationType, CommonListResponse,
                          CommonResponse};
use infra::session::RedisStore;
use util::generate_random_id;
use server::Server;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceLoginForm {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceLoginResponse {
    pub sid: String,
    pub resource_id: String,
}

impl<'r> Responder<'r> for ResourceLoginResponse {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

#[post("/login", data = "<input>")]
pub fn login(
    input: Json<ResourceLoginForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<ResourceLoginResponse, ed::Error> {
    let form = input.into_inner();
    let name = form.name;
    let password = form.password;
    let service = server.resource_service();
    let ret = service.log_in(&name, &password)?;
    let mut sid = generate_random_id(64usize);
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            sid = token.clone();
            redis_store.set(&sid, constant::RESOURCE_SESS_ID_FIELD, &ret.id)?;
        } else {
            redis_store.set(&sid, constant::RESOURCE_SESS_ID_FIELD, &ret.id)?;
        }
    } else {
        redis_store.set(&sid, constant::RESOURCE_SESS_ID_FIELD, &ret.id)?;
    }
    Ok(ResourceLoginResponse {
        sid,
        resource_id: ret.id.clone(),
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
pub fn get_resources<'r>(
    cors: Guard<'r>,
    server: Server,
) -> rocket_cors::Responder<Result<CommonListResponse<ResourceRepr>, ed::Error>> {
    let service = server.resource_service();
    cors.responder(
        service
            .get_resources()
            .map(|v| CommonListResponse { list: v }),
    )
}

#[get("/<id>")]
pub fn get_resource<'r>(
    cors: Guard<'r>,
    id: String,
    server: Server,
) -> rocket_cors::Responder<Result<ResourceRepr, ed::Error>> {
    let service = server.resource_service();
    cors.responder(service.get_resource(&id))
}

#[get("/<id>/detail")]
pub fn get_detailed_resource(
    id: String,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<DetailedResourceRepr, ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let resource_self_id = redis_store.get(&token, constant::RESOURCE_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let service = server.resource_service();
            return service.get_detailed_resource(&id, &resource_self_id, &admin_id);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceRegisterForm {
    pub name: String,
    pub password: String,
    pub website: String,
    pub scope: Vec<Scope>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceRegisterResponse {
    pub resource_id: String,
}

impl<'r> Responder<'r> for ResourceRegisterResponse {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

#[post("/", data = "<input>")]
pub fn register_resource<'r>(
    cors: Guard<'r>,
    input: Json<ResourceRegisterForm>,
    server: Server,
) -> rocket_cors::Responder<Result<ResourceRegisterResponse, ed::Error>> {
    let form = input.into_inner();
    let cmd = RegisterResourceCmd {
        name: form.name,
        password: form.password,
        website: form.website,
        scope: form.scope,
    };
    let service = server.resource_service();
    cors.responder(
        service
            .register_resource(&cmd)
            .map(|r| ResourceRegisterResponse { resource_id: r.id }),
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceUpdateForm {
    pub name: String,
    pub new_password: Option<String>,
    pub website: String,
    pub scope: Vec<Scope>,
    pub current_password: Option<String>,
}

#[put("/<id>", data = "<input>")]
pub fn update_resource(
    id: String,
    input: Json<ResourceUpdateForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<(), ed::Error> {
    let form = input.into_inner();
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let resource_self_id = redis_store.get(&token, constant::RESOURCE_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let cmd = UpdateResourceCmd {
                target_id: id,
                self_id: resource_self_id,
                admin_id,
                name: form.name,
                new_password: form.new_password,
                website: form.website,
                scope: form.scope,
                current_password: form.current_password,
            };
            let service = server.resource_service();
            return service.update_resource(&cmd);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

#[delete("/<id>")]
pub fn delete_resource(
    id: String,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<(), ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let resource_self_id = redis_store.get(&token, constant::RESOURCE_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let service = server.resource_service();
            return service.delete_resource(&id, &resource_self_id, &admin_id);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}
