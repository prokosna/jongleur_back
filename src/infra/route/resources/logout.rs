use rocket::http::{Cookie, Cookies};

use domain::consts;
use infra::http::DomainResponder;

#[get("/logout")]
pub fn get_logout(mut cookies: Cookies) -> DomainResponder {
    cookies.remove(Cookie::named(consts::COOKIE_KEY));
    DomainResponder::ok(None)
}
