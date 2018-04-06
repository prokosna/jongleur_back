use chrono::prelude::*;
use domain::error::domain as ed;
use domain::model::{Client, ClientType};
use domain::repository::{AdminRepository, AdminRepositoryComponent, ClientRepository,
                         ClientRepositoryComponent, ResourceRepository,
                         ResourceRepositoryComponent};

pub struct RegisterClientCmd {
    pub name: String,
    pub password: String,
    pub website: String,
    pub client_type: String,
    pub redirect_uris: Vec<String>,
    pub resource_id: String,
}

pub struct UpdateClientCmd {
    pub target_id: String,
    pub self_id: Option<String>,
    pub admin_id: Option<String>,
    pub name: Option<String>,
    pub new_password: Option<String>,
    pub website: Option<String>,
    pub client_type: Option<String>,
    pub redirect_uris: Option<Vec<String>>,
    pub resource_id: Option<String>,
    pub current_password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientRepr {
    pub id: String,
    pub name: String,
    pub website: String,
    pub resource_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ClientRepr {
    fn from_client(client: &Client) -> Self {
        ClientRepr {
            id: client.id.clone(),
            name: client.name.clone(),
            website: client.website.clone(),
            resource_id: client.resource_id.clone(),
            created_at: client.created_at.clone(),
            updated_at: client.updated_at.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedClientRepr {
    pub id: String,
    pub name: String,
    pub website: String,
    pub client_type: ClientType,
    pub client_secret: String,
    pub redirect_uris: Vec<String>,
    pub resource_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DetailedClientRepr {
    fn from_client(client: &Client) -> Self {
        DetailedClientRepr {
            id: client.id.clone(),
            name: client.name.clone(),
            website: client.website.clone(),
            client_type: client.client_type.clone(),
            client_secret: client.client_secret.clone(),
            redirect_uris: client.redirect_uris.clone(),
            resource_id: client.resource_id.clone(),
            created_at: client.created_at.clone(),
            updated_at: client.updated_at.clone(),
        }
    }
}

pub trait ClientService:
    AdminRepositoryComponent + ClientRepositoryComponent + ResourceRepositoryComponent
{
    fn log_in(&self, name: &String, password: &String) -> Result<ClientRepr, ed::Error> {
        let repository = self.client_repository();
        let client = repository.find_by_name(name)?;
        if let Some(v) = client {
            if v.is_authenticated_by_password(password) {
                return Ok(ClientRepr::from_client(&v));
            }
        };
        Err(ed::ErrorKind::LoginFailed(format!("Name => {}", name)).into())
    }

    fn get_clients(&self) -> Result<Vec<ClientRepr>, ed::Error> {
        let repository = self.client_repository();
        repository
            .find_all()
            .map(|v| v.iter().map(|r| ClientRepr::from_client(&r)).collect())
    }

    fn get_client(&self, id: &String) -> Result<ClientRepr, ed::Error> {
        let repository = self.client_repository();
        match repository.find_by_id(id)? {
            Some(client) => Ok(ClientRepr::from_client(&client)),
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", id)).into()),
        }
    }

    fn get_detailed_client(
        &self,
        target_id: &String,
        self_id: &Option<String>,
        admin_id: &Option<String>,
    ) -> Result<DetailedClientRepr, ed::Error> {
        let repository = self.client_repository();
        match repository.find_by_id(target_id)? {
            Some(client) => {
                if self_id.is_some() && self_id.as_ref().unwrap() == &client.id {
                    return Ok(DetailedClientRepr::from_client(&client));
                } else if admin_id.is_some() {
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        return Ok(DetailedClientRepr::from_client(&client));
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
            }
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", target_id)).into()),
        }
    }

    fn register_client(&self, cmd: &RegisterClientCmd) -> Result<DetailedClientRepr, ed::Error> {
        let repository = self.client_repository();
        let resource_repository = self.resource_repository();
        if resource_repository.find_by_id(&cmd.resource_id)?.is_none() {
            return Err(ed::ErrorKind::EntityNotFound(format!(
                "Resource not found. ID => {}",
                cmd.resource_id
            )).into());
        }
        let client = Client::builder(&cmd.name, &cmd.password, &cmd.website, &cmd.resource_id)
            .client_type(&ClientType::new(&cmd.client_type)?)
            .redirect_uris(&cmd.redirect_uris)
            .build();
        repository.add(&client)?;
        Ok(DetailedClientRepr::from_client(&client))
    }

    fn update_client(&self, cmd: &UpdateClientCmd) -> Result<(), ed::Error> {
        let repository = self.client_repository();
        let resource_repository = self.resource_repository();
        match repository.find_by_id(&cmd.target_id)? {
            Some(mut client) => {
                if cmd.self_id.is_some() && cmd.self_id.as_ref().unwrap() == &client.id {
                    // `Client` updates itself
                    if cmd.name.is_some() {
                        client.name = cmd.name.as_ref().unwrap().clone();
                    }
                    if cmd.website.is_some() {
                        client.website = cmd.website.as_ref().unwrap().clone();
                    }
                    if cmd.client_type.is_some() {
                        client.client_type = ClientType::new(cmd.client_type.as_ref().unwrap())?;
                    }
                    if cmd.redirect_uris.is_some() {
                        client.redirect_uris = cmd.redirect_uris.as_ref().unwrap().clone();
                    }
                    if cmd.resource_id.is_some() {
                        let resource_id = cmd.resource_id.as_ref().unwrap().clone();
                        if resource_repository.find_by_id(&resource_id)?.is_none() {
                            return Err(ed::ErrorKind::EntityNotFound(format!(
                                "Resource not found. ID => {}",
                                resource_id
                            )).into());
                        }
                        client.resource_id = resource_id;
                    }
                    if cmd.new_password.is_some() {
                        client.update_password(
                            cmd.new_password.as_ref().unwrap(),
                            cmd.current_password.as_ref().unwrap(),
                        )?;
                    }
                    client.update_timestamp();
                    return repository.update(&client);
                } else if cmd.admin_id.is_some() {
                    // `Admin` updates the client
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(cmd.admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        if cmd.name.is_some() {
                            client.name = cmd.name.as_ref().unwrap().clone();
                        }
                        if cmd.website.is_some() {
                            client.website = cmd.website.as_ref().unwrap().clone();
                        }
                        if cmd.client_type.is_some() {
                            client.client_type =
                                ClientType::new(cmd.client_type.as_ref().unwrap())?;
                        }
                        if cmd.redirect_uris.is_some() {
                            client.redirect_uris = cmd.redirect_uris.as_ref().unwrap().clone();
                        }
                        if cmd.resource_id.is_some() {
                            let resource_id = cmd.resource_id.as_ref().unwrap().clone();
                            if resource_repository.find_by_id(&resource_id)?.is_none() {
                                return Err(ed::ErrorKind::EntityNotFound(format!(
                                    "Resource not found. ID => {}",
                                    resource_id
                                )).into());
                            }
                            client.resource_id = resource_id;
                        }
                        client.update_timestamp();
                        return repository.update(&client);
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", cmd.target_id)).into());
            }
            None => {
                Err(ed::ErrorKind::EntityNotFound(format!("Not found. {}", cmd.target_id)).into())
            }
        }
    }

    fn delete_client(
        &self,
        target_id: &String,
        self_id: &Option<String>,
        admin_id: &Option<String>,
    ) -> Result<(), ed::Error> {
        let repository = self.client_repository();
        match repository.find_by_id(target_id)? {
            Some(client) => {
                if self_id.is_some() && self_id.as_ref().unwrap() == &client.id {
                    return repository.remove(client);
                } else if admin_id.is_some() {
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        return repository.remove(client);
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
            }
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", target_id)).into()),
        }
    }
}

pub trait ClientServiceComponent {
    type ClientService: ClientService;
    fn client_service(&self) -> &Self::ClientService;
}

// Implement
impl<T: AdminRepositoryComponent + ClientRepositoryComponent + ResourceRepositoryComponent>
    ClientService for T
{
}
