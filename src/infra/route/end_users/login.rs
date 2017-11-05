use rocket::http::{Cookie, Cookies};
use rocket_contrib::Json;

use application::end_users::EndUserService;
use domain::consts;
use domain::session::Store;
use infra::db::{MongoClient, MongoRepository};
use infra::session::{RedisClient, RedisStore};
use infra::http::DomainResponder;
use util::generate_uid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserLoginForm {
    name: String,
    password: String,
}

#[post("/login", data = "<input>")]
pub fn post_login(
    input: Json<EndUserLoginForm>,
    redis_client: RedisClient,
    mongo_client: MongoClient,
    mut cookies: Cookies,
) -> DomainResponder {
    let login_form = input.into_inner();
    let name = login_form.name;
    let password = login_form.password;
    let store = RedisStore::new(&redis_client);
    let repository = MongoRepository::new(&mongo_client);
    let end_user_service = EndUserService::<RedisStore, MongoRepository>::new();
    let ret = end_user_service
        .log_in_end_user(&name, &password, &repository)
        .and_then(|end_user| {
            let sid = cookies
                .get(consts::COOKIE_KEY)
                .map(|ref c| c.value().to_string())
                .unwrap_or(generate_uid(64usize).unwrap());
            store.set(&sid, consts::END_USER_SESS_ID_FIELD, &end_user.id)?;
            Ok((sid, end_user.id.clone()))
        });
    match ret {
        Ok((sid, id)) => {
            cookies.add(Cookie::build(consts::COOKIE_KEY, sid).path("/").finish());
            DomainResponder::logged_in(id)
        }
        Err(e) => DomainResponder::from_domain_error(e),
    }
}
