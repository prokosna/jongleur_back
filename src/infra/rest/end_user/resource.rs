use actix_web::*;
use app::end_user::{DetailedEndUserRepr, EndUserRepr, EndUserService, EndUserServiceComponent,
                    RegisterEndUserCmd, UpdateEndUserCmd};
use constant;
use domain::error::domain as ed;
use infra::rest::common::{CommonListResponse, CommonResponse, HttpStatus};
use infra::rest::middleware::AuthorizationType;
use server::ApplicationState;
use util::{convert_str_to_naive_date, generate_random_id};

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

impl Responder for EndUserLoginResponse {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

pub fn login(
    req: HttpRequest<ApplicationState>,
    form: Json<EndUserLoginForm>,
) -> Result<EndUserLoginResponse, ed::Error> {
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let name = &form.name;
    let password = &form.password;
    let service = server.end_user_service();
    let ret = service.log_in(name, password)?;
    let sid = generate_random_id(64usize);
    redis_store.set(&sid, constant::END_USER_SESS_ID_FIELD, &ret.id)?;
    Ok(EndUserLoginResponse {
        sid,
        end_user_id: ret.id.clone(),
    })
}

pub fn logout(req: HttpRequest<ApplicationState>) -> Result<HttpResponse, ed::Error> {
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        redis_store.del(&token, Some(constant::END_USER_SESS_ID_FIELD))?
    }
    Ok(HttpResponse::Ok().finish())
}

pub fn get_end_users(
    req: HttpRequest<ApplicationState>,
) -> Result<CommonListResponse<EndUserRepr>, ed::Error> {
    let server = &req.state().server;
    let service = server.end_user_service();
    service
        .get_end_users()
        .map(|v| CommonListResponse { list: v })
}

pub fn get_end_user(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<EndUserRepr, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let service = server.end_user_service();
    service.get_end_user(&id)
}

pub fn get_detailed_end_user(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
) -> Result<DetailedEndUserRepr, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let end_user_self_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let service = server.end_user_service();
        return service.get_detailed_end_user(&id, &end_user_self_id, &admin_id);
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

impl Responder for EndUserRegisterResponse {
    type Item = HttpResponse;
    type Error = Error;
    fn respond_to(self, _req: HttpRequest) -> Result<HttpResponse, Error> {
        Ok(CommonResponse::respond(&self, HttpStatus::ok()))
    }
}

pub fn register_end_user(
    req: HttpRequest<ApplicationState>,
    form: Json<EndUserRegisterForm>,
) -> Result<EndUserRegisterResponse, ed::Error> {
    let server = &req.state().server;
    // Parse ISO8601 timestamp e.g. 2000-1-1T00:00:00.000+09:00
    let birthdate = match form.birthdate
        .clone()
        .map_or(Ok(None), |d| convert_str_to_naive_date(&d).map(|v| Some(v)))
    {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let cmd = RegisterEndUserCmd {
        name: form.name.clone(),
        password: form.password.clone(),
        email: form.email.clone(),
        given_name: form.given_name.clone(),
        family_name: form.family_name.clone(),
        middle_name: form.middle_name.clone(),
        nickname: form.nickname.clone(),
        profile: form.profile.clone(),
        picture: form.picture.clone(),
        website: form.website.clone(),
        gender: form.gender.clone(),
        birthdate,
        zoneinfo: form.zoneinfo.clone(),
        locale: form.locale.clone(),
        phone_number: form.phone_number.clone(),
    };
    let service = server.end_user_service();
    service
        .register_end_user(&cmd)
        .map(|r| EndUserRegisterResponse { end_user_id: r.id })
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserUpdateForm {
    pub name: Option<String>,
    pub email: Option<String>,
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
    pub new_password: Option<String>,
    pub current_password: Option<String>,
}

pub fn update_end_user(
    req: HttpRequest<ApplicationState>,
    path: Path<(String)>,
    form: Json<EndUserUpdateForm>,
) -> Result<HttpResponse, ed::Error> {
    let id = path.into_inner();
    let server = &req.state().server;
    let redis_store = &req.state().redis_pool.get_store()?;
    let auth = req.clone()
        .extensions()
        .get::<AuthorizationType>()
        .map(|v| v.clone());
    if let Some(AuthorizationType::Bearer { token }) = auth {
        let end_user_self_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let cmd = UpdateEndUserCmd {
            target_id: id,
            self_id: end_user_self_id,
            admin_id,
            name: form.name.clone(),
            email: form.email.clone(),
            new_password: form.new_password.clone(),
            current_password: form.current_password.clone(),
            given_name: form.given_name.clone(),
            family_name: form.family_name.clone(),
            middle_name: form.middle_name.clone(),
            nickname: form.nickname.clone(),
            profile: form.profile.clone(),
            picture: form.picture.clone(),
            website: form.website.clone(),
            gender: form.gender.clone(),
            birthdate: form.birthdate
                .clone()
                .map_or(Ok(None), |d| convert_str_to_naive_date(&d).map(|v| Some(v)))?,
            zoneinfo: form.zoneinfo.clone(),
            locale: form.locale.clone(),
            phone_number: form.phone_number.clone(),
        };
        let service = server.end_user_service();
        return service
            .update_end_user(&cmd)
            .map(|()| HttpResponse::Ok().finish());
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}

pub fn delete_end_user(
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
        let end_user_self_id = redis_store.get(&token, constant::END_USER_SESS_ID_FIELD)?;
        let admin_id = redis_store.get(&token, constant::ADMIN_SESS_ID_FIELD)?;
        let service = server.end_user_service();
        return service
            .delete_end_user(&id, &end_user_self_id, &admin_id)
            .map(|()| HttpResponse::Ok().finish());
    }
    Err(ed::ErrorKind::RequireLogin(format!("ID => {}", id)).into())
}
