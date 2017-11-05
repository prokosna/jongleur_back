use rocket_cors::Cors;

pub fn get_cors_options() -> Cors {
    Cors {
        ..Default::default()
    }
}
