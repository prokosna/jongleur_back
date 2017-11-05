use rocket::http::{Cookie, Cookies};
use rocket_contrib::Json;

use application::clients::ClientService;
use domain::consts;
use domain::model::Client;
use domain::session::Store;
use infra::db::{MongoClient, MongoRepository};
use infra::session::{RedisClient, RedisStore};
use infra::http::DomainResponder;
use util::generate_uid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientLoginForm {
    name: String,
    password: String,
}

#[post("/login", data = "<input>")]
pub fn post_login(
    input: Json<ClientLoginForm>,
    redis_client: RedisClient,
    mongo_client: MongoClient,
    mut cookies: Cookies,
) -> DomainResponder {
    let login_form = input.into_inner();
    let name = login_form.name;
    let password = login_form.password;
    let store = RedisStore::new(&redis_client);
    let repository = MongoRepository::new(&mongo_client);
    let client_service = ClientService::<RedisStore, MongoRepository>::new();
    let ret = client_service
        .log_in_client(&name, &password, &repository)
        .and_then(|client: Client| {
            let sid = cookies
                .get(consts::COOKIE_KEY)
                .map(|ref c| c.value().to_string())
                .unwrap_or(generate_uid(64usize).unwrap());
            store.set(&sid, consts::CLIENT_SESS_ID_FIELD, &client.id)?;
            Ok((sid, client.id.to_string()))
        });
    match ret {
        Ok((sid, id)) => {
            cookies.add(Cookie::build(consts::COOKIE_KEY, sid).path("/").finish());
            DomainResponder::logged_in(id)
        }
        Err(e) => DomainResponder::from_domain_error(e),
    }
}
