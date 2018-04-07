use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::Json;
use rocket_cors::{self, Guard};

use app::client::{ClientRepr, ClientService, ClientServiceComponent, DetailedClientRepr,
                  GetClientsCmd, RegisterClientCmd, UpdateClientCmd};
use constant;
use domain::error::domain as ed;
use infra::rest::common::{AuthorizationHeader, AuthorizationType, CommonListResponse,
                          CommonResponse};
use infra::session::RedisStore;
use server::Server;
use util::generate_random_id;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientLoginForm {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientLoginResponse {
    pub sid: String,
    pub client_id: String,
}

#[derive(Deserialize, Debug, FromForm)]
pub struct GetClientsParams {
    pub resource_id: Option<String>,
}

impl<'r> Responder<'r> for ClientLoginResponse {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

#[post("/login", data = "<input>")]
pub fn login(
    input: Json<ClientLoginForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<ClientLoginResponse, ed::Error> {
    let form = input.into_inner();
    let name = form.name;
    let password = form.password;
    let service = server.client_service();
    let ret = service.log_in(&name, &password)?;
    let mut sid = generate_random_id(64usize);
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            sid = token.clone();
            redis_store.set(&sid, constant::CLIENT_SESS_ID_FIELD, &ret.id)?;
        } else {
            redis_store.set(&sid, constant::CLIENT_SESS_ID_FIELD, &ret.id)?;
        }
    } else {
        redis_store.set(&sid, constant::CLIENT_SESS_ID_FIELD, &ret.id)?;
    }
    Ok(ClientLoginResponse {
        sid,
        client_id: ret.id.clone(),
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
pub fn get_clients<'r>(
    cors: Guard<'r>,
    server: Server,
) -> rocket_cors::Responder<Result<CommonListResponse<ClientRepr>, ed::Error>> {
    let service = server.client_service();
    let cmd = GetClientsCmd { resource_id: None };
    cors.responder(
        service
            .get_clients(cmd)
            .map(|v| CommonListResponse { list: v }),
    )
}

#[get("/?<get_clients_params>")]
pub fn get_clients_with_params<'r>(
    cors: Guard<'r>,
    get_clients_params: GetClientsParams,
    server: Server,
) -> rocket_cors::Responder<Result<CommonListResponse<ClientRepr>, ed::Error>> {
    let service = server.client_service();
    let cmd = GetClientsCmd {
        resource_id: get_clients_params.resource_id.clone(),
    };
    cors.responder(
        service
            .get_clients(cmd)
            .map(|v| CommonListResponse { list: v }),
    )
}

#[get("/<id>")]
pub fn get_client<'r>(
    cors: Guard<'r>,
    id: String,
    server: Server,
) -> rocket_cors::Responder<Result<ClientRepr, ed::Error>> {
    let service = server.client_service();
    cors.responder(service.get_client(&id))
}

#[get("/<id>/detail")]
pub fn get_detailed_client(
    id: String,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<DetailedClientRepr, ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let client_self_id = redis_store.get(&token, constant::CLIENT_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let service = server.client_service();
            return service.get_detailed_client(&id, &client_self_id, &admin_id);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientRegisterForm {
    pub name: String,
    pub password: String,
    pub website: String,
    pub client_type: String,
    pub redirect_uris: Vec<String>,
    pub resource_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientRegisterResponse {
    pub client_id: String,
}

impl<'r> Responder<'r> for ClientRegisterResponse {
    fn respond_to(self, _request: &Request) -> Result<Response<'r>, Status> {
        CommonResponse::respond(&self, Status::Ok).ok()
    }
}

#[post("/", data = "<input>")]
pub fn register_client<'r>(
    cors: Guard<'r>,
    input: Json<ClientRegisterForm>,
    server: Server,
) -> rocket_cors::Responder<Result<ClientRegisterResponse, ed::Error>> {
    let form = input.into_inner();
    let cmd = RegisterClientCmd {
        name: form.name,
        password: form.password,
        website: form.website,
        client_type: form.client_type,
        redirect_uris: form.redirect_uris,
        resource_id: form.resource_id,
    };
    let service = server.client_service();
    cors.responder(
        service
            .register_client(&cmd)
            .map(|r| ClientRegisterResponse { client_id: r.id }),
    )
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientUpdateForm {
    pub name: Option<String>,
    pub website: Option<String>,
    pub client_type: Option<String>,
    pub redirect_uris: Option<Vec<String>>,
    pub resource_id: Option<String>,
    pub new_password: Option<String>,
    pub current_password: Option<String>,
}

#[put("/<id>", data = "<input>")]
pub fn update_client(
    id: String,
    input: Json<ClientUpdateForm>,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<(), ed::Error> {
    let form = input.into_inner();
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let client_self_id = redis_store.get(&token, constant::CLIENT_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let cmd = UpdateClientCmd {
                target_id: id,
                self_id: client_self_id,
                admin_id,
                name: form.name,
                new_password: form.new_password,
                website: form.website,
                client_type: form.client_type,
                redirect_uris: form.redirect_uris,
                resource_id: form.resource_id,
                current_password: form.current_password,
            };
            let service = server.client_service();
            return service.update_client(&cmd);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

#[delete("/<id>")]
pub fn delete_client(
    id: String,
    authorization_header: AuthorizationHeader,
    redis_store: RedisStore,
    server: Server,
) -> Result<(), ed::Error> {
    if let AuthorizationType::Bearer = authorization_header.auth_type {
        if let Some(token) = authorization_header.token {
            let client_self_id = redis_store.get(&token, constant::CLIENT_SESS_ID_FIELD)?;
            let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
            let service = server.client_service();
            return service.delete_client(&id, &client_self_id, &admin_id);
        }
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}
