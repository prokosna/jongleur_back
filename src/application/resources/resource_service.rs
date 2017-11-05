use std::marker::PhantomData;

use domain::session::Store;
use domain::repository::Repository;
use domain::model::{Resource, ResourceBuilder, Scope};
use domain::consts;
use domain::error::domain as ed;

pub struct ResourceService<S, V>
where
    S: Store,
    V: Repository<Resource>,
{
    _phantom1: PhantomData<S>,
    _phantom2: PhantomData<V>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceRegisterForm {
    name: String,
    password: String,
    website: String,
    scope: Vec<Scope>,
}

impl<S, V> ResourceService<S, V>
where
    S: Store,
    V: Repository<Resource>,
{
    pub fn new() -> Self {
        ResourceService {
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn log_in_resource(
        &self,
        name: &String,
        password: &String,
        repository: &V,
    ) -> Result<Resource, ed::Error> {
        // find the resource
        let query = doc! {"name" => name};
        let mut ret: Vec<Resource> = repository.find(&query)?;
        if ret.len() != 1 {
            return Err(ed::ErrorKind::LoginFailed.into());
        }
        let resource: Resource = ret.remove(0usize);
        if !resource.authenticate(&password) {
            return Err(ed::ErrorKind::LoginFailed.into());
        }
        Ok(resource)
    }

    pub fn get_resources(&self, repository: &V) -> Result<Vec<Resource>, ed::Error> {
        let query = doc! {"is_valid" => true};
        repository.find(&query)
    }

    pub fn get_resource(&self, id: &String, repository: &V) -> Result<Resource, ed::Error> {
        let resource: Resource = repository
            .find_by_key(id)
            .and_then(|r| r.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        Ok(resource)
    }

    pub fn get_private_resource(
        &self,
        id: &String,
        sid: &String,
        store: &S,
        repository: &V,
    ) -> Result<Resource, ed::Error> {
        let resource_id: String = store.get(&sid, consts::RESOURCE_SESS_ID_FIELD)?;
        if id != &resource_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private resource.".to_string())
                    .into(),
            );
        }
        let resource: Resource = repository
            .find_by_key(&resource_id)
            .and_then(|r| r.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        Ok(resource)
    }

    pub fn register_resource(
        &self,
        register_form: ResourceRegisterForm,
        repository: &V,
    ) -> Result<(), ed::Error> {
        let mut resource_builder = ResourceBuilder::new(
            register_form.name,
            register_form.password,
            register_form.website,
        );
        resource_builder = resource_builder.scope(register_form.scope.clone());
        let resource = resource_builder.build().unwrap();
        repository.insert(&resource)?;
        Ok(())
    }

    pub fn update_private_resource(
        &self,
        id: &String,
        sid: &String,
        register_form: ResourceRegisterForm,
        store: &S,
        repository: &V,
    ) -> Result<(), ed::Error> {
        let resource_id: String = store.get(&sid, consts::RESOURCE_SESS_ID_FIELD)?;
        if id != &resource_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private resource.".to_string())
                    .into(),
            );
        }
        let old_resource: Resource = repository
            .find_by_key(&resource_id)
            .and_then(|r| r.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        let mut resource_builder = ResourceBuilder::new(
            register_form.name,
            register_form.password,
            register_form.website,
        );
        resource_builder = resource_builder.scope(register_form.scope.clone());
        let mut resource = resource_builder.build().unwrap();
        resource.id = old_resource.id;
        resource.created_at = old_resource.created_at;
        resource.is_valid = old_resource.is_valid;
        repository.update(&resource)?;
        Ok(())
    }

    pub fn delete_private_resource(
        &self,
        id: &String,
        sid: &String,
        store: &S,
        repository: &V,
    ) -> Result<(), ed::Error> {
        let resource_id: String = store.get(&sid, consts::RESOURCE_SESS_ID_FIELD)?;
        if id != &resource_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private resource.".to_string())
                    .into(),
            );
        }
        repository.remove_by_key(&resource_id)
    }
}
