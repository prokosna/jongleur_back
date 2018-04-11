use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Json;
use actix_web::Path;
use actix_web::Responder;
use app::admin::{AdminRepr, AdminService, AdminServiceComponent, RegisterAdminCmd, UpdateAdminCmd};
use constant;
use domain::error::domain as ed;
use infra::rest::common::{CommonListResponse, CommonResponse, HttpStatus};
use infra::rest::middleware::AuthorizationType;
use server::ApplicationState;
use util::generate_random_id;

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

impl Responder for AdminLoginResponse {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

pub fn login(
    req: HttpRequest<ApplicationState>,
    form: Json<AdminLoginForm>,
) -> Result<AdminLoginResponse, ed::Error> {
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let name = &form.name;
    let password = &form.password;
    let service = server.admin_service();
    let ret = service.log_in(name, password)?;
    let sid = generate_random_id(64usize);
    redis_store.set(&sid, constant::ADMIN_SESS_ID_FIELD, &ret.id)?;
    Ok(AdminLoginResponse {
        sid,
        admin_id: ret.id.clone(),
    })
}

pub fn logout(req: HttpRequest<ApplicationState>) -> Result<HttpResponse, ed::Error> {
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        redis_store.del(&token, Some(constant::ADMIN_SESS_ID_FIELD))?
    }
    Ok(HttpResponse::Ok().finish())
}

pub fn get_admins(
    req: HttpRequest<ApplicationState>,
) -> Result<CommonListResponse<AdminRepr>, ed::Error> {
    let server = &req.state().server;
    let service = server.admin_service();
    service.get_admins().map(|v| CommonListResponse { list: v })
}

pub fn get_admin(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<AdminRepr, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let self_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let service = server.admin_service();
        return service.get_admin(&id, &self_id);
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

impl Responder for AdminRegisterResponse {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

pub fn register_admin(
    req: HttpRequest<ApplicationState>,
    form: Json<AdminRegisterForm>,
) -> Result<AdminRegisterResponse, ed::Error> {
    let server = &req.state().server;
    let cmd = RegisterAdminCmd {
        name: form.name.clone(),
        password: form.password.clone(),
    };
    let service = server.admin_service();
    service
        .register_admin(&cmd)
        .map(|r| AdminRegisterResponse { admin_id: r.id })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminUpdateForm {
    pub name: Option<String>,
    pub new_password: Option<String>,
    pub current_password: Option<String>,
}

pub fn update_admin(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
    form: Json<AdminUpdateForm>,
) -> Result<&'static str, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let self_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let cmd = UpdateAdminCmd {
            target_id: id,
            self_id,
            name: form.name.clone(),
            new_password: form.new_password.clone(),
            current_password: form.current_password.clone(),
        };
        let service = server.admin_service();
        return service.update_admin(&cmd).map(|()| "");
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

pub fn delete_admin(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<&'static str, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let self_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let service = server.admin_service();
        return service.delete_admin(&id, &self_id).map(|()| "");
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}
