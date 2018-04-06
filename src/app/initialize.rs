use config::AppConfig;
use domain::model::Admin;
use domain::repository::{AdminRepository, AdminRepositoryComponent};

pub trait InitializeService: AdminRepositoryComponent {
    fn initialize(&self) {
        let repository = self.admin_repository();

        if let Ok(o) = repository.find_by_name(&"admin".to_string()) {
            if o.is_none() {
                let admin =
                    Admin::builder(&"admin".to_string(), &AppConfig::default_admin_password())
                        .build();
                repository.add(&admin).unwrap();
                info!(
                    "There was no default admin user: admin was created with the default password"
                );
            }
        } else {
            panic!()
        }
    }
}

pub trait InitializeServiceComponent {
    type InitializeService: InitializeService;
    fn initialize_service(&self) -> &Self::InitializeService;
}

impl<T: AdminRepositoryComponent> InitializeService for T {}
