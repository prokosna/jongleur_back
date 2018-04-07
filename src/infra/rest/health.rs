use rocket_cors::{self, Guard};

use app::health::{HealthService, HealthServiceComponent};
use domain::error::domain as ed;
use server::Server;

#[get("/")]
pub fn get_health<'a>(
    cors: Guard<'a>,
    server: Server,
) -> rocket_cors::Responder<Result<String, ed::Error>> {
    let service = server.health_service();
    cors.responder(service.check().map(|()| "Ok".to_string()))
}
