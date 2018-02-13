use chrono::prelude::*;

use domain::error::domain as ed;
use domain::model::{Resource, Scope};
use domain::repository::{AdminRepository, AdminRepositoryComponent, ResourceRepository,
                         ResourceRepositoryComponent};

pub struct RegisterResourceCmd {
    pub name: String,
    pub password: String,
    pub website: String,
    pub scope: Vec<Scope>,
}

pub struct UpdateResourceCmd {
    pub target_id: String,
    pub self_id: Option<String>,
    pub admin_id: Option<String>,
    pub name: String,
    pub new_password: Option<String>,
    pub website: String,
    pub scope: Vec<Scope>,
    pub current_password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceRepr {
    pub id: String,
    pub name: String,
    pub website: String,
    pub scope: Vec<Scope>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ResourceRepr {
    fn from_resource(resource: &Resource) -> Self {
        ResourceRepr {
            id: resource.id.clone(),
            name: resource.name.clone(),
            website: resource.website.clone(),
            scope: resource.scope.clone(),
            created_at: resource.created_at.clone(),
            updated_at: resource.updated_at.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedResourceRepr {
    pub id: String,
    pub name: String,
    pub website: String,
    pub scope: Vec<Scope>,
    pub resource_secret: String,
    pub created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl DetailedResourceRepr {
    fn from_resource(resource: &Resource) -> Self {
        DetailedResourceRepr {
            id: resource.id.clone(),
            name: resource.name.clone(),
            website: resource.website.clone(),
            scope: resource.scope.clone(),
            resource_secret: resource.resource_secret.clone(),
            created_at: resource.created_at.clone(),
            updated_at: resource.updated_at.clone(),
        }
    }
}

pub trait ResourceService
    : AdminRepositoryComponent + ResourceRepositoryComponent {
    fn log_in(&self, name: &String, password: &String) -> Result<ResourceRepr, ed::Error> {
        let repository = self.resource_repository();
        let resource = repository.find_by_name(name)?;
        if let Some(v) = resource {
            if v.is_authenticated(password) {
                return Ok(ResourceRepr::from_resource(&v));
            }
        };
        Err(ed::ErrorKind::LoginFailed(format!("Name => {}", name)).into())
    }

    fn get_resources(&self) -> Result<Vec<ResourceRepr>, ed::Error> {
        let repository = self.resource_repository();
        repository
            .find_all()
            .map(|v| v.iter().map(|r| ResourceRepr::from_resource(&r)).collect())
    }

    fn get_resource(&self, id: &String) -> Result<ResourceRepr, ed::Error> {
        let repository = self.resource_repository();
        match repository.find_by_id(id)? {
            Some(resource) => Ok(ResourceRepr::from_resource(&resource)),
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", id)).into()),
        }
    }

    fn get_detailed_resource(
        &self,
        target_id: &String,
        self_id: &Option<String>,
        admin_id: &Option<String>,
    ) -> Result<DetailedResourceRepr, ed::Error> {
        let repository = self.resource_repository();
        match repository.find_by_id(target_id)? {
            Some(resource) => {
                if self_id.is_some() && self_id.as_ref().unwrap() == &resource.id {
                    return Ok(DetailedResourceRepr::from_resource(&resource));
                } else if admin_id.is_some() {
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        return Ok(DetailedResourceRepr::from_resource(&resource));
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
            }
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", target_id)).into()),
        }
    }

    fn register_resource(
        &self,
        cmd: &RegisterResourceCmd,
    ) -> Result<DetailedResourceRepr, ed::Error> {
        let repository = self.resource_repository();
        let resource = Resource::builder(&cmd.name, &cmd.password, &cmd.website)
            .scope(&cmd.scope)
            .build();
        repository.add(&resource)?;
        Ok(DetailedResourceRepr::from_resource(&resource))
    }

    fn update_resource(&self, cmd: &UpdateResourceCmd) -> Result<(), ed::Error> {
        let repository = self.resource_repository();
        match repository.find_by_id(&cmd.target_id)? {
            Some(mut resource) => {
                if cmd.self_id.is_some() && cmd.self_id.as_ref().unwrap() == &resource.id {
                    // `Resource` updates itself
                    if !cmd.current_password.is_some()
                        || !resource.is_authenticated(cmd.current_password.as_ref().unwrap())
                    {
                        return Err(
                            ed::ErrorKind::WrongPassword(format!("ID => {}", resource.id)).into(),
                        );
                    }
                    resource.name = cmd.name.clone();
                    resource.website = cmd.website.clone();
                    resource.scope = cmd.scope.clone();
                    if cmd.new_password.is_some() {
                        resource.update_password(
                            cmd.new_password.as_ref().unwrap(),
                            cmd.current_password.as_ref().unwrap(),
                        )?;
                    }
                    resource.update_timestamp();
                    return repository.update(&resource);
                } else if cmd.admin_id.is_some() {
                    // `Admin` updates the resource
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(cmd.admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        resource.name = cmd.name.clone();
                        resource.website = cmd.website.clone();
                        resource.scope = cmd.scope.clone();
                        resource.update_timestamp();
                        return repository.update(&resource);
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", cmd.target_id)).into());
            }
            None => {
                Err(ed::ErrorKind::EntityNotFound(format!("Not found. {}", cmd.target_id)).into())
            }
        }
    }

    fn delete_resource(
        &self,
        target_id: &String,
        self_id: &Option<String>,
        admin_id: &Option<String>,
    ) -> Result<(), ed::Error> {
        let repository = self.resource_repository();
        match repository.find_by_id(target_id)? {
            Some(resource) => {
                if self_id.is_some() && self_id.as_ref().unwrap() == &resource.id {
                    return repository.remove(resource);
                } else if admin_id.is_some() {
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        return repository.remove(resource);
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
            }
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", target_id)).into()),
        }
    }
}

pub trait ResourceServiceComponent {
    type ResourceService: ResourceService;
    fn resource_service(&self) -> &Self::ResourceService;
}

// Implement
impl<T: AdminRepositoryComponent + ResourceRepositoryComponent> ResourceService for T {}
