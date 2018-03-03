use chrono::prelude::*;

use domain::error::domain as ed;
use domain::model::Admin;
use domain::repository::{AdminRepository, AdminRepositoryComponent};

pub struct RegisterAdminCmd {
    pub name: String,
    pub password: String,
}

pub struct UpdateAdminCmd {
    pub target_id: String,
    pub self_id: Option<String>,
    pub name: Option<String>,
    pub new_password: Option<String>,
    pub current_password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdminRepr {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminRepr {
    fn from_admin(admin: &Admin) -> Self {
        AdminRepr {
            id: admin.id.clone(),
            name: admin.name.clone(),
            created_at: admin.created_at.clone(),
            updated_at: admin.updated_at.clone(),
        }
    }
}

pub trait AdminService: AdminRepositoryComponent {
    fn log_in(&self, name: &String, password: &String) -> Result<AdminRepr, ed::Error> {
        let repository = self.admin_repository();
        let admin = repository.find_by_name(name)?;
        if let Some(v) = admin {
            if v.is_authenticated(password) {
                return Ok(AdminRepr::from_admin(&v));
            }
        };
        Err(ed::ErrorKind::LoginFailed(format!("Name => {}", name)).into())
    }

    fn get_admin(
        &self,
        target_id: &String,
        self_id: &Option<String>,
    ) -> Result<AdminRepr, ed::Error> {
        if self_id.is_none() {
            return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
        }
        let repository = self.admin_repository();
        match repository.find_by_id(target_id)? {
            Some(admin) => {
                if self_id.as_ref().unwrap() == &admin.id {
                    return Ok(AdminRepr::from_admin(&admin));
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
            }
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", target_id)).into()),
        }
    }

    fn register_admin(&self, cmd: &RegisterAdminCmd) -> Result<AdminRepr, ed::Error> {
        let repository = self.admin_repository();
        let admin = Admin::builder(&cmd.name, &cmd.password).build();
        repository.add(&admin)?;
        Ok(AdminRepr::from_admin(&admin))
    }

    fn update_admin(&self, cmd: &UpdateAdminCmd) -> Result<(), ed::Error> {
        let repository = self.admin_repository();
        match repository.find_by_id(&cmd.target_id)? {
            Some(mut admin) => {
                if cmd.self_id.is_some() && cmd.self_id.as_ref().unwrap() == &admin.id {
                    if cmd.name.is_some() {
                        admin.name = cmd.name.as_ref().unwrap().clone();
                    }
                    if cmd.new_password.is_some() {
                        admin.update_password(
                            cmd.new_password.as_ref().unwrap(),
                            &cmd.current_password.as_ref().unwrap(),
                        )?;
                    }
                    admin.update_timestamp();
                    return repository.update(&admin);
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", cmd.target_id)).into());
            }
            None => {
                Err(ed::ErrorKind::EntityNotFound(format!("Not found. {}", cmd.target_id)).into())
            }
        }
    }

    fn delete_admin(&self, target_id: &String, self_id: &Option<String>) -> Result<(), ed::Error> {
        let repository = self.admin_repository();
        match repository.find_by_id(target_id)? {
            Some(admin) => {
                if self_id.is_some() && self_id.as_ref().unwrap() == &admin.id {
                    return repository.remove(admin);
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
            }
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", target_id)).into()),
        }
    }
}

pub trait AdminServiceComponent {
    type AdminService: AdminService;
    fn admin_service(&self) -> &Self::AdminService;
}

// Implement
impl<T: AdminRepositoryComponent + AdminRepositoryComponent> AdminService for T {}
