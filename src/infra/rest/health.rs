use actix_web::*;
use app::health::{HealthService, HealthServiceComponent};
use domain::error::domain as ed;
use server::ApplicationState;

pub fn get_health(req: HttpRequest<ApplicationState>) -> Result<&'static str, ed::Error> {
    let server = &req.state().server;
    let service = server.health_service();
    service.check().map(|()| "Ok")
}
