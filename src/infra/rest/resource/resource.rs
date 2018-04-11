use actix_web::*;

use app::resource::{DetailedResourceRepr, RegisterResourceCmd, ResourceRepr, ResourceService,
                    ResourceServiceComponent, UpdateResourceCmd};
use constant;
use domain::error::domain as ed;
use domain::model::Scope;
use infra::rest::common::{CommonListResponse, CommonResponse, HttpStatus};
use infra::rest::middleware::AuthorizationType;
use server::ApplicationState;
use util::generate_random_id;

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

impl Responder for ResourceLoginResponse {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

pub fn login(
    req: HttpRequest<ApplicationState>,
    form: Json<ResourceLoginForm>,
) -> Result<ResourceLoginResponse, ed::Error> {
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let name = &form.name;
    let password = &form.password;
    let service = server.resource_service();
    let ret = service.log_in(name, password)?;
    let sid = generate_random_id(64usize);
    redis_store.set(&sid, constant::RESOURCE_SESS_ID_FIELD, &ret.id)?;
    Ok(ResourceLoginResponse {
        sid,
        resource_id: ret.id.clone(),
    })
}

pub fn logout(req: HttpRequest<ApplicationState>) -> Result<HttpResponse, ed::Error> {
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        redis_store.del(&token, Some(constant::RESOURCE_SESS_ID_FIELD))?
    }
    Ok(HttpResponse::Ok().finish())
}

pub fn get_resources(
    req: HttpRequest<ApplicationState>,
) -> Result<CommonListResponse<ResourceRepr>, ed::Error> {
    let server = &req.state().server;
    let service = server.resource_service();
    service
        .get_resources()
        .map(|v| CommonListResponse { list: v })
}

pub fn get_resource(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<ResourceRepr, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let service = server.resource_service();
    service.get_resource(&id)
}

pub fn get_detailed_resource(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<DetailedResourceRepr, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let resource_self_id = redis_store.get(&token, constant::RESOURCE_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let service = server.resource_service();
        return service.get_detailed_resource(&id, &resource_self_id, &admin_id);
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

impl Responder for ResourceRegisterResponse {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

pub fn register_resource(
    req: HttpRequest<ApplicationState>,
    form: Json<ResourceRegisterForm>,
) -> Result<ResourceRegisterResponse, ed::Error> {
    let server = &req.state().server;
    let cmd = RegisterResourceCmd {
        name: form.name.clone(),
        password: form.password.clone(),
        website: form.website.clone(),
        scope: form.scope.clone(),
    };
    let service = server.resource_service();
    service
        .register_resource(&cmd)
        .map(|r| ResourceRegisterResponse { resource_id: r.id })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceUpdateForm {
    pub name: Option<String>,
    pub website: Option<String>,
    pub scope: Option<Vec<Scope>>,
    pub new_password: Option<String>,
    pub current_password: Option<String>,
}

pub fn update_resource(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
    form: Json<ResourceUpdateForm>,
) -> Result<HttpResponse, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let resource_self_id = redis_store.get(&token, constant::RESOURCE_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let cmd = UpdateResourceCmd {
            target_id: id,
            self_id: resource_self_id,
            admin_id,
            name: form.name.clone(),
            new_password: form.new_password.clone(),
            website: form.website.clone(),
            scope: form.scope.clone(),
            current_password: form.current_password.clone(),
        };
        let service = server.resource_service();
        return service
            .update_resource(&cmd)
            .map(|()| HttpResponse::Ok().finish());
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

pub fn delete_resource(
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
        let resource_self_id = redis_store.get(&token, constant::RESOURCE_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let service = server.resource_service();
        return service
            .delete_resource(&id, &resource_self_id, &admin_id)
            .map(|()| HttpResponse::Ok().finish());
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}
