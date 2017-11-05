use std::marker::PhantomData;

use domain::session::Store;
use domain::repository::Repository;
use domain::model::{Client, ClientBuilder, ClientType};
use domain::consts;
use domain::error::domain as ed;

pub struct ClientService<S, C>
where
    S: Store,
    C: Repository<Client>,
{
    _phantom1: PhantomData<S>,
    _phantom2: PhantomData<C>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientRegisterForm {
    name: String,
    password: String,
    website: String,
    client_type: Option<String>,
    redirect_uris: Vec<String>,
    resource_id: String,
}

impl<S, C> ClientService<S, C>
where
    S: Store,
    C: Repository<Client>,
{
    pub fn new() -> Self {
        ClientService {
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn log_in_client(
        &self,
        name: &String,
        password: &String,
        repository: &C,
    ) -> Result<Client, ed::Error> {
        // find the client
        let query = doc! {"name" => name};
        let mut ret: Vec<Client> = repository.find(&query)?;
        if ret.len() != 1 {
            return Err(ed::ErrorKind::LoginFailed.into());
        }
        let client: Client = ret.remove(0usize);
        if !client.authenticate_by_password(&password) {
            return Err(ed::ErrorKind::LoginFailed.into());
        }
        Ok(client)
    }

    pub fn get_clients(&self, repository: &C) -> Result<Vec<Client>, ed::Error> {
        let query = doc! {"is_valid" => true};
        repository.find(&query)
    }

    pub fn get_client(&self, id: &String, repository: &C) -> Result<Client, ed::Error> {
        let client: Client = repository
            .find_by_key(id)
            .and_then(|c| c.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        Ok(client)
    }

    pub fn get_private_client(
        &self,
        id: &String,
        sid: &String,
        store: &S,
        repository: &C,
    ) -> Result<Client, ed::Error> {
        let client_id: String = store.get(&sid, consts::CLIENT_SESS_ID_FIELD)?;
        if id != &client_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private client.".to_string())
                    .into(),
            );
        }
        let client: Client = repository
            .find_by_key(&client_id)
            .and_then(|c| c.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        Ok(client)
    }

    pub fn register_client(
        &self,
        register_form: ClientRegisterForm,
        repository: &C,
    ) -> Result<(), ed::Error> {
        let mut client_builder = ClientBuilder::new(
            register_form.name,
            register_form.password,
            register_form.website,
            register_form.resource_id,
        );
        if let Some(client_type) = register_form.client_type {
            if let Ok(client_type) = ClientType::new(&client_type) {
                client_builder = client_builder.client_type(client_type);
            } else {
                return Err(
                    ed::ErrorKind::InvalidRequest(
                        format!("Unsupported client type: {}", client_type),
                    ).into(),
                );
            }
        }
        client_builder = client_builder.redirect_uris(register_form.redirect_uris.clone());
        let client = client_builder.build().unwrap();
        repository.insert(&client)?;
        Ok(())
    }

    pub fn update_private_client(
        &self,
        id: &String,
        sid: &String,
        register_form: ClientRegisterForm,
        store: &S,
        repository: &C,
    ) -> Result<(), ed::Error> {
        let client_id: String = store.get(&sid, consts::CLIENT_SESS_ID_FIELD)?;
        if id != &client_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private client.".to_string())
                    .into(),
            );
        }
        let old_client: Client = repository
            .find_by_key(&client_id)
            .and_then(|c| c.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        let mut client_builder = ClientBuilder::new(
            register_form.name,
            register_form.password,
            register_form.website,
            register_form.resource_id,
        );
        if let Some(client_type) = register_form.client_type {
            if let Ok(client_type) = ClientType::new(&client_type) {
                client_builder = client_builder.client_type(client_type);
            } else {
                return Err(
                    ed::ErrorKind::InvalidRequest(
                        format!("Unsupported client type: {}", client_type),
                    ).into(),
                );
            }
        }
        client_builder = client_builder.redirect_uris(register_form.redirect_uris.clone());
        let mut client = client_builder.build().unwrap();
        client.id = old_client.id;
        client.created_at = old_client.created_at;
        client.is_valid = old_client.is_valid;
        repository.update(&client)?;
        Ok(())
    }

    pub fn delete_private_client(
        &self,
        id: &String,
        sid: &String,
        store: &S,
        repository: &C,
    ) -> Result<(), ed::Error> {
        let client_id: String = store.get(&sid, consts::CLIENT_SESS_ID_FIELD)?;
        if id != &client_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private client.".to_string())
                    .into(),
            );
        }
        repository.remove_by_key(&client_id)
    }
}
