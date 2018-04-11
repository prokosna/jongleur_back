use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Json;
use actix_web::Path;
use actix_web::Query;
use actix_web::Responder;
use app::client::{ClientRepr, ClientService, ClientServiceComponent, DetailedClientRepr,
                  GetClientsCmd, RegisterClientCmd, UpdateClientCmd};
use constant;
use domain::error::domain as ed;
use infra::rest::common::{CommonListResponse, CommonResponse, HttpStatus};
use infra::rest::middleware::AuthorizationType;
use server::ApplicationState;
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

#[derive(Deserialize, Debug)]
pub struct GetClientsParams {
    pub resource_id: Option<String>,
}

impl Responder for ClientLoginResponse {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

pub fn login(
    req: HttpRequest<ApplicationState>,
    form: Json<ClientLoginForm>,
) -> Result<ClientLoginResponse, ed::Error> {
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let name = &form.name;
    let password = &form.password;
    let service = server.client_service();
    let ret = service.log_in(name, password)?;
    let sid = generate_random_id(64usize);
    redis_store.set(&sid, constant::CLIENT_SESS_ID_FIELD, &ret.id)?;
    Ok(ClientLoginResponse {
        sid,
        client_id: ret.id.clone(),
    })
}

pub fn logout(req: HttpRequest<ApplicationState>) -> Result<HttpResponse, ed::Error> {
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        redis_store.del(&token, Some(constant::CLIENT_SESS_ID_FIELD))?
    }
    Ok(HttpResponse::Ok().finish())
}

#[derive(Deserialize)]
pub struct GetClientsQuery {
    pub resource_id: Option<String>,
}

pub fn get_clients(
    req: HttpRequest<ApplicationState>,
    query: Query<GetClientsQuery>,
) -> Result<CommonListResponse<ClientRepr>, ed::Error> {
    let server = &req.state().server;
    let service = server.client_service();
    let cmd = GetClientsCmd {
        resource_id: query.resource_id.clone(),
    };
    service
        .get_clients(cmd)
        .map(|v| CommonListResponse { list: v })
}

pub fn get_client(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<ClientRepr, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let service = server.client_service();
    service.get_client(&id)
}

pub fn get_detailed_client(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<DetailedClientRepr, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let client_self_id = redis_store.get(&token, constant::CLIENT_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let service = server.client_service();
        return service.get_detailed_client(&id, &client_self_id, &admin_id);
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

impl Responder for ClientRegisterResponse {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

pub fn register_client<'r>(
    req: HttpRequest<ApplicationState>,
    form: Json<ClientRegisterForm>,
) -> Result<ClientRegisterResponse, ed::Error> {
    let server = &req.state().server;
    let cmd = RegisterClientCmd {
        name: form.name.clone(),
        password: form.password.clone(),
        website: form.website.clone(),
        client_type: form.client_type.clone(),
        redirect_uris: form.redirect_uris.clone(),
        resource_id: form.resource_id.clone(),
    };
    let service = server.client_service();
    service
        .register_client(&cmd)
        .map(|r| ClientRegisterResponse { client_id: r.id })
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

pub fn update_client(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
    form: Json<ClientUpdateForm>,
) -> Result<HttpResponse, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let client_self_id = redis_store.get(&token, constant::CLIENT_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let cmd = UpdateClientCmd {
            target_id: id,
            self_id: client_self_id,
            admin_id,
            name: form.name.clone(),
            new_password: form.new_password.clone(),
            website: form.website.clone(),
            client_type: form.client_type.clone(),
            redirect_uris: form.redirect_uris.clone(),
            resource_id: form.resource_id.clone(),
            current_password: form.current_password.clone(),
        };
        let service = server.client_service();
        return service
            .update_client(&cmd)
            .map(|()| HttpResponse::Ok().finish());
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

pub fn delete_client(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<HttpResponse, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let client_self_id = redis_store.get(&token, constant::CLIENT_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let service = server.client_service();
        return service
            .delete_client(&id, &client_self_id, &admin_id)
            .map(|()| HttpResponse::Ok().finish());
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}
