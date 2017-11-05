use chrono::prelude::*;
use std::marker::PhantomData;

use domain::consts;
use domain::session::Store;
use domain::repository::Repository;
use domain::model::{EndUser, EndUserBuilder};
use domain::error::domain as ed;

pub struct EndUserService<T, U>
where
    T: Store,
    U: Repository<EndUser>,
{
    _phantom1: PhantomData<T>,
    _phantom2: PhantomData<U>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EndUserRegisterForm {
    name: String,
    password: String,
    email: String,
    given_name: Option<String>,
    family_name: Option<String>,
    middle_name: Option<String>,
    nickname: Option<String>,
    profile: Option<String>,
    picture: Option<String>,
    website: Option<String>,
    gender: Option<String>,
    birthdate: Option<String>,
    zoneinfo: Option<String>,
    locale: Option<String>,
    phone_number: Option<String>,
}

impl<T, U> EndUserService<T, U>
where
    T: Store,
    U: Repository<EndUser>,
{
    pub fn new() -> Self {
        EndUserService {
            _phantom1: PhantomData,
            _phantom2: PhantomData,
        }
    }

    pub fn log_in_end_user(
        &self,
        name: &String,
        password: &String,
        repository: &U,
    ) -> Result<EndUser, ed::Error> {
        let query = doc! {"name" => name};
        let mut ret: Vec<EndUser> = repository.find(&query)?;
        if ret.len() != 1 {
            return Err(ed::ErrorKind::LoginFailed.into());
        }
        let mut end_user = ret.remove(0usize);
        if !end_user.authenticate(&password) {
            return Err(ed::ErrorKind::LoginFailed.into());
        }
        repository.update(&end_user)?;
        Ok(end_user)
    }

    pub fn get_end_users(&self, repository: &U) -> Result<Vec<EndUser>, ed::Error> {
        let query = doc! {"is_valid" => true};
        repository.find(&query)
    }

    pub fn get_end_user(&self, id: &String, repository: &U) -> Result<EndUser, ed::Error> {
        let end_user: EndUser = repository
            .find_by_key(id)
            .and_then(|u| u.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        Ok(end_user)
    }

    pub fn get_private_end_user(
        &self,
        id: &String,
        sid: &String,
        store: &T,
        repository: &U,
    ) -> Result<EndUser, ed::Error> {
        let end_user_id = store.get(&sid, consts::END_USER_SESS_ID_FIELD)?;
        if id != &end_user_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private end-user.".to_string())
                    .into(),
            );
        }
        let end_user: EndUser = repository
            .find_by_key(&end_user_id)
            .and_then(|u| u.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        Ok(end_user)
    }

    pub fn register_end_user(
        &self,
        register_form: EndUserRegisterForm,
        repository: &U,
    ) -> Result<(), ed::Error> {
        let mut end_user_builder = EndUserBuilder::new(
            register_form.name,
            register_form.password,
            register_form.email,
        );
        end_user_builder = end_user_builder
            .given_name(register_form.given_name)
            .family_name(register_form.family_name)
            .middle_name(register_form.middle_name)
            .nickname(register_form.nickname)
            .profile(register_form.profile)
            .picture(register_form.picture)
            .website(register_form.website)
            .gender(register_form.gender)
            .zoneinfo(register_form.zoneinfo)
            .locale(register_form.locale)
            .phone_number(register_form.phone_number);
        if let Some(ref birthdate) = register_form.birthdate {
            if let Ok(date) = NaiveDate::parse_from_str(birthdate, "%+") {
                end_user_builder = end_user_builder.birthdate(Some(date));
            } else {
                return Err(
                    ed::ErrorKind::InvalidRequest(
                        format!("birthdate must be ISO8601/RFC3339: {}", birthdate),
                    ).into(),
                );
            }
        }
        let end_user = end_user_builder.build().unwrap();
        repository.insert(&end_user)?;
        Ok(())
    }

    pub fn update_private_end_user(
        &self,
        id: &String,
        sid: &String,
        register_form: EndUserRegisterForm,
        store: &T,
        repository: &U,
    ) -> Result<(), ed::Error> {
        let end_user_id: String = store.get(&sid, consts::END_USER_SESS_ID_FIELD)?;
        if id != &end_user_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private end-user.".to_string())
                    .into(),
            );
        }
        let old_end_user: EndUser = repository
            .find_by_key(&end_user_id)
            .and_then(|u| u.ok_or(ed::ErrorKind::EntryNotFound.into()))?;
        let mut end_user_builder = EndUserBuilder::new(
            register_form.name,
            register_form.password,
            register_form.email,
        );
        end_user_builder = end_user_builder
            .given_name(register_form.given_name)
            .family_name(register_form.family_name)
            .middle_name(register_form.middle_name)
            .nickname(register_form.nickname)
            .profile(register_form.profile)
            .picture(register_form.picture)
            .website(register_form.website)
            .gender(register_form.gender)
            .zoneinfo(register_form.zoneinfo)
            .locale(register_form.locale)
            .phone_number(register_form.phone_number);
        if let Some(ref birthdate) = register_form.birthdate {
            if let Ok(date) = NaiveDate::parse_from_str(birthdate, "%+") {
                end_user_builder = end_user_builder.birthdate(Some(date));
            } else {
                return Err(
                    ed::ErrorKind::InvalidRequest(
                        format!("birthdate must be ISO8601/RFC3339: {}", birthdate),
                    ).into(),
                );
            }
        }
        let mut end_user = end_user_builder.build().unwrap();
        end_user.id = old_end_user.id;
        end_user.accepted_clients = old_end_user.accepted_clients;
        end_user.created_at = old_end_user.created_at;
        end_user.last_authenticated_at = old_end_user.last_authenticated_at;
        end_user.is_valid = old_end_user.is_valid;
        end_user.phone_number_verified = old_end_user.phone_number_verified;
        if end_user.phone_number != old_end_user.phone_number {
            end_user.phone_number_verified = false;
        }
        repository.update(&end_user)?;
        Ok(())
    }

    pub fn delete_private_end_user(
        &self,
        id: &String,
        sid: &String,
        store: &T,
        repository: &U,
    ) -> Result<(), ed::Error> {
        let end_user_id: String = store.get(&sid, consts::END_USER_SESS_ID_FIELD)?;
        if id != &end_user_id {
            return Err(
                ed::ErrorKind::AccessDenied("You cannot access the private end-user.".to_string())
                    .into(),
            );
        }
        repository.remove_by_key(&end_user_id)
    }
}
