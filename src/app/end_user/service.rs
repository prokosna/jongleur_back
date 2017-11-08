use chrono::prelude::*;

use domain::error::domain as ed;
use domain::model::{AcceptedClient, EndUser};
use domain::repository::{AdminRepository, AdminRepositoryComponent, EndUserRepository,
                         EndUserRepositoryComponent};

pub struct RegisterEndUserCmd {
    pub name: String,
    pub password: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub nickname: Option<String>,
    pub profile: Option<String>,
    pub picture: Option<String>,
    pub website: Option<String>,
    pub gender: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub zoneinfo: Option<String>,
    pub locale: Option<String>,
    pub phone_number: Option<String>,
}

pub struct UpdateEndUserCmd {
    pub target_id: String,
    pub self_id: Option<String>,
    pub admin_id: Option<String>,
    pub name: String,
    pub email: String,
    pub new_password: Option<String>,
    pub current_password: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub nickname: Option<String>,
    pub profile: Option<String>,
    pub picture: Option<String>,
    pub website: Option<String>,
    pub gender: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub zoneinfo: Option<String>,
    pub locale: Option<String>,
    pub phone_number: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserRepr {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl EndUserRepr {
    fn from_end_user(end_user: &EndUser) -> Self {
        EndUserRepr {
            id: end_user.id.clone(),
            name: end_user.name.clone(),
            created_at: end_user.created_at.clone(),
            updated_at: end_user.updated_at.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DetailedEndUserRepr {
    pub id: String,
    pub name: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub middle_name: Option<String>,
    pub nickname: Option<String>,
    pub profile: Option<String>,
    pub picture: Option<String>,
    pub website: Option<String>,
    pub gender: Option<String>,
    pub birthdate: Option<NaiveDate>,
    pub zoneinfo: Option<String>,
    pub locale: Option<String>,
    pub phone_number: Option<String>,
    pub accepted_clients: Vec<AcceptedClient>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DetailedEndUserRepr {
    fn from_end_user(end_user: &EndUser) -> Self {
        DetailedEndUserRepr {
            id: end_user.id.clone(),
            name: end_user.name.clone(),
            email: end_user.email.clone(),
            given_name: end_user.given_name.clone(),
            family_name: end_user.family_name.clone(),
            middle_name: end_user.middle_name.clone(),
            nickname: end_user.nickname.clone(),
            profile: end_user.profile.clone(),
            picture: end_user.picture.clone(),
            website: end_user.website.clone(),
            gender: end_user.gender.clone(),
            birthdate: end_user.birthdate.clone(),
            zoneinfo: end_user.zoneinfo.clone(),
            locale: end_user.locale.clone(),
            phone_number: end_user.phone_number.clone(),
            accepted_clients: end_user.accepted_clients.clone(),
            created_at: end_user.created_at.clone(),
            updated_at: end_user.updated_at.clone(),
        }
    }
}

pub trait EndUserService: AdminRepositoryComponent + EndUserRepositoryComponent {
    fn log_in(&self, name: &String, password: &String) -> Result<EndUserRepr, ed::Error> {
        let repository = self.end_user_repository();
        let end_user = repository.find_by_name(name)?;
        if let Some(v) = end_user {
            if v.is_authenticated(password) {
                return Ok(EndUserRepr::from_end_user(&v));
            }
        };
        Err(ed::ErrorKind::LoginFailed(format!("Name => {}", name)).into())
    }

    fn get_end_users(&self) -> Result<Vec<EndUserRepr>, ed::Error> {
        let repository = self.end_user_repository();
        repository
            .find_all()
            .map(|v| v.iter().map(|r| EndUserRepr::from_end_user(&r)).collect())
    }

    fn get_end_user(&self, id: &String) -> Result<EndUserRepr, ed::Error> {
        let repository = self.end_user_repository();
        match repository.find_by_id(id)? {
            Some(end_user) => Ok(EndUserRepr::from_end_user(&end_user)),
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", id)).into()),
        }
    }

    fn get_detailed_end_user(
        &self,
        target_id: &String,
        self_id: &Option<String>,
        admin_id: &Option<String>,
    ) -> Result<DetailedEndUserRepr, ed::Error> {
        let repository = self.end_user_repository();
        match repository.find_by_id(target_id)? {
            Some(end_user) => {
                if self_id.is_some() && self_id.as_ref().unwrap() == &end_user.id {
                    return Ok(DetailedEndUserRepr::from_end_user(&end_user));
                } else if admin_id.is_some() {
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        return Ok(DetailedEndUserRepr::from_end_user(&end_user));
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
            }
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", target_id)).into()),
        }
    }

    fn register_end_user(
        &self,
        cmd: &RegisterEndUserCmd,
    ) -> Result<DetailedEndUserRepr, ed::Error> {
        let repository = self.end_user_repository();
        let end_user = EndUser::builder(&cmd.name, &cmd.password, &cmd.email)
            .given_name(&cmd.given_name)
            .family_name(&cmd.family_name)
            .middle_name(&cmd.middle_name)
            .nickname(&cmd.nickname)
            .profile(&cmd.profile)
            .picture(&cmd.picture)
            .website(&cmd.website)
            .gender(&cmd.gender)
            .birthdate(&cmd.birthdate)
            .zoneinfo(&cmd.zoneinfo)
            .locale(&cmd.locale)
            .phone_number(&cmd.phone_number)
            .build();
        repository.add(&end_user)?;
        Ok(DetailedEndUserRepr::from_end_user(&end_user))
    }

    fn update_end_user(&self, cmd: &UpdateEndUserCmd) -> Result<(), ed::Error> {
        let repository = self.end_user_repository();
        match repository.find_by_id(&cmd.target_id)? {
            Some(mut end_user) => {
                if cmd.self_id.is_some() && cmd.self_id.as_ref().unwrap() == &end_user.id {
                    // `EndUser` updates itself
                    if !cmd.current_password.is_some()
                        || !end_user.is_authenticated(cmd.current_password.as_ref().unwrap())
                    {
                        return Err(
                            ed::ErrorKind::WrongPassword(format!("ID => {}", end_user.id)).into(),
                        );
                    }
                    end_user.name = cmd.name.clone();
                    end_user.email = cmd.email.clone();
                    end_user.given_name = cmd.given_name.clone();
                    end_user.family_name = cmd.family_name.clone();
                    end_user.middle_name = cmd.middle_name.clone();
                    end_user.nickname = cmd.nickname.clone();
                    end_user.profile = cmd.profile.clone();
                    end_user.picture = cmd.picture.clone();
                    end_user.website = cmd.website.clone();
                    end_user.gender = cmd.gender.clone();
                    end_user.birthdate = cmd.birthdate.clone();
                    end_user.zoneinfo = cmd.zoneinfo.clone();
                    end_user.locale = cmd.locale.clone();
                    end_user.phone_number = cmd.phone_number.clone();
                    if cmd.new_password.is_some() {
                        end_user.update_password(
                            cmd.new_password.as_ref().unwrap(),
                            cmd.current_password.as_ref().unwrap(),
                        )?;
                    }
                    end_user.update_timestamp();
                    return repository.update(&end_user);
                } else if cmd.admin_id.is_some() {
                    // `Admin` updates the end_user
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(cmd.admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        end_user.name = cmd.name.clone();
                        end_user.email = cmd.email.clone();
                        end_user.given_name = cmd.given_name.clone();
                        end_user.family_name = cmd.family_name.clone();
                        end_user.middle_name = cmd.middle_name.clone();
                        end_user.nickname = cmd.nickname.clone();
                        end_user.profile = cmd.profile.clone();
                        end_user.picture = cmd.picture.clone();
                        end_user.website = cmd.website.clone();
                        end_user.gender = cmd.gender.clone();
                        end_user.birthdate = cmd.birthdate.clone();
                        end_user.zoneinfo = cmd.zoneinfo.clone();
                        end_user.locale = cmd.locale.clone();
                        end_user.phone_number = cmd.phone_number.clone();
                        end_user.update_timestamp();
                        return repository.update(&end_user);
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", cmd.target_id)).into());
            }
            None => {
                Err(ed::ErrorKind::EntityNotFound(format!("Not found. {}", cmd.target_id)).into())
            }
        }
    }

    fn delete_end_user(
        &self,
        target_id: &String,
        self_id: &Option<String>,
        admin_id: &Option<String>,
    ) -> Result<(), ed::Error> {
        let repository = self.end_user_repository();
        match repository.find_by_id(target_id)? {
            Some(end_user) => {
                if self_id.is_some() && self_id.as_ref().unwrap() == &end_user.id {
                    return repository.remove(end_user);
                } else if admin_id.is_some() {
                    let admin_repository = self.admin_repository();
                    if admin_repository
                        .find_by_id(admin_id.as_ref().unwrap())?
                        .is_some()
                    {
                        return repository.remove(end_user);
                    }
                }
                return Err(ed::ErrorKind::AccessDenied(format!("ID => {}", target_id)).into());
            }
            None => Err(ed::ErrorKind::EntityNotFound(format!("ID => {}", target_id)).into()),
        }
    }
}

pub trait EndUserServiceComponent {
    type EndUserService: EndUserService;
    fn end_user_service(&self) -> &Self::EndUserService;
}

// Implement
impl<T: AdminRepositoryComponent + EndUserRepositoryComponent> EndUserService for T {}
