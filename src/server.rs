use app::admin::AdminServiceComponent;
use app::client::ClientServiceComponent;
use app::end_user::EndUserServiceComponent;
use app::health::HealthServiceComponent;
use app::initialize::InitializeServiceComponent;
use app::oidc::OidcServiceComponent;
use app::resource::ResourceServiceComponent;
use domain::repository::{AccessTokenRepositoryComponent, AdminRepositoryComponent,
                         ClientRepositoryComponent, EndUserRepositoryComponent,
                         GrantRepositoryComponent, HealthRepositoryComponent,
                         IdTokenRepositoryComponent, RefreshTokenRepositoryComponent,
                         ResourceRepositoryComponent};
use domain::service::{AuthorizeServiceComponent, ClientCredentialsServiceComponent,
                      IntrospectServiceComponent, KeyServiceComponent,
                      RefreshTokenServiceComponent,
                      ResourceOwnerPasswordCredentialsServiceComponent, UserinfoServiceComponent};
use infra::persistence::{AccessTokenRepositoryMongo, AdminRepositoryMongo, ClientRepositoryMongo,
                         EndUserRepositoryMongo, GrantRepositoryMongo, HealthRepositoryMongo,
                         IdTokenRepositoryMongo, MongoClient, RefreshTokenRepositoryMongo,
                         ResourceRepositoryMongo};
use infra::session::RedisPool;
use mongo_driver::client::{ClientPool, Uri};
use std::sync::Arc;

#[derive(Clone)]
pub struct Server {
    access_token_repository: AccessTokenRepositoryMongo,
    admin_repository: AdminRepositoryMongo,
    client_repository: ClientRepositoryMongo,
    end_user_repository: EndUserRepositoryMongo,
    grant_repository: GrantRepositoryMongo,
    health_repository: HealthRepositoryMongo,
    id_token_repository: IdTokenRepositoryMongo,
    refresh_token_repository: RefreshTokenRepositoryMongo,
    resource_repository: ResourceRepositoryMongo,
}

// Dependency injection
// Repositories
impl AccessTokenRepositoryComponent for Server {
    type AccessTokenRepository = AccessTokenRepositoryMongo;

    fn access_token_repository(&self) -> &Self::AccessTokenRepository {
        &self.access_token_repository
    }
}

impl AdminRepositoryComponent for Server {
    type AdminRepository = AdminRepositoryMongo;

    fn admin_repository(&self) -> &Self::AdminRepository {
        &self.admin_repository
    }
}

impl ClientRepositoryComponent for Server {
    type ClientRepository = ClientRepositoryMongo;

    fn client_repository(&self) -> &Self::ClientRepository {
        &self.client_repository
    }
}

impl EndUserRepositoryComponent for Server {
    type EndUserRepository = EndUserRepositoryMongo;

    fn end_user_repository(&self) -> &Self::EndUserRepository {
        &self.end_user_repository
    }
}

impl GrantRepositoryComponent for Server {
    type GrantRepository = GrantRepositoryMongo;

    fn grant_repository(&self) -> &Self::GrantRepository {
        &self.grant_repository
    }
}

impl HealthRepositoryComponent for Server {
    type HealthRepository = HealthRepositoryMongo;

    fn health_repository(&self) -> &Self::HealthRepository {
        &self.health_repository
    }
}

impl IdTokenRepositoryComponent for Server {
    type IdTokenRepository = IdTokenRepositoryMongo;

    fn id_token_repository(&self) -> &Self::IdTokenRepository {
        &self.id_token_repository
    }
}

impl RefreshTokenRepositoryComponent for Server {
    type RefreshTokenRepository = RefreshTokenRepositoryMongo;

    fn refresh_token_repository(&self) -> &Self::RefreshTokenRepository {
        &self.refresh_token_repository
    }
}

impl ResourceRepositoryComponent for Server {
    type ResourceRepository = ResourceRepositoryMongo;

    fn resource_repository(&self) -> &Self::ResourceRepository {
        &self.resource_repository
    }
}

// Domain Services
impl AuthorizeServiceComponent for Server {
    type AuthorizeService = Self;

    fn authorize_service(&self) -> &Self::AuthorizeService {
        self
    }
}

impl ClientCredentialsServiceComponent for Server {
    type ClientCredentialsService = Self;

    fn client_credentials_service(&self) -> &Self::ClientCredentialsService {
        self
    }
}

impl IntrospectServiceComponent for Server {
    type IntrospectService = Self;

    fn introspect_service(&self) -> &Self::IntrospectService {
        self
    }
}

impl KeyServiceComponent for Server {
    type KeyService = Self;

    fn key_service(&self) -> &Self::KeyService {
        self
    }
}

impl RefreshTokenServiceComponent for Server {
    type RefreshTokenService = Self;

    fn refresh_token_service(&self) -> &Self::RefreshTokenService {
        self
    }
}

impl ResourceOwnerPasswordCredentialsServiceComponent for Server {
    type ResourceOwnerPasswordCredentialsService = Self;

    fn resource_owner_password_credentials_service(
        &self,
    ) -> &Self::ResourceOwnerPasswordCredentialsService {
        self
    }
}

impl UserinfoServiceComponent for Server {
    type UserinfoService = Self;

    fn userinfo_service(&self) -> &Self::UserinfoService {
        self
    }
}

// Application Services
impl AdminServiceComponent for Server {
    type AdminService = Self;

    fn admin_service(&self) -> &Self::AdminService {
        self
    }
}

impl ClientServiceComponent for Server {
    type ClientService = Self;

    fn client_service(&self) -> &Self::ClientService {
        self
    }
}

impl EndUserServiceComponent for Server {
    type EndUserService = Self;

    fn end_user_service(&self) -> &Self::EndUserService {
        self
    }
}

impl OidcServiceComponent for Server {
    type OidcService = Self;

    fn oidc_service(&self) -> &Self::OidcService {
        self
    }
}

impl ResourceServiceComponent for Server {
    type ResourceService = Self;

    fn resource_service(&self) -> &Self::ResourceService {
        self
    }
}

impl InitializeServiceComponent for Server {
    type InitializeService = Self;

    fn initialize_service(&self) -> &Self::InitializeService {
        self
    }
}

impl HealthServiceComponent for Server {
    type HealthService = Self;

    fn health_service(&self) -> &Self::HealthService {
        self
    }
}

// For actix-web shared state
#[derive(Clone)]
pub struct ApplicationState {
    pub server: Server,
    pub redis_pool: RedisPool,
}

impl ApplicationState {
    pub fn new() -> ApplicationState {
        let s = build_server();
        let p = RedisPool::new();
        ApplicationState {
            server: s,
            redis_pool: p,
        }
    }
}

fn build_server() -> Server {
    let db_name = "jongleur".to_string();
    let uri = Uri::new("mongodb://localhost:27017/").unwrap();
    let pool = Arc::new(ClientPool::new(uri.clone(), None));
    Server {
        access_token_repository: AccessTokenRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
        admin_repository: AdminRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
        client_repository: ClientRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
        end_user_repository: EndUserRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
        grant_repository: GrantRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
        health_repository: HealthRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
        id_token_repository: IdTokenRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
        refresh_token_repository: RefreshTokenRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
        resource_repository: ResourceRepositoryMongo {
            mongo_client: MongoClient::new(&db_name, pool.clone()),
        },
    }
}
