use serde::{Deserialize, Serialize};
use fake::Dummy;
use std::{fmt, time::SystemTime};

mod models;

pub use models::*;

#[derive(Debug, Serialize, Deserialize, Clone,Dummy)]
pub enum Gender {
    Male,
    Female,
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<Gender> for String {
    fn from(val: Gender) -> Self {
        match val {
            Gender::Male => "Male1".to_string(),
            Gender::Female => "Female1".to_string(),
        }
    }
}

impl From<String> for Gender {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Male,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl User {
    pub fn new(id: String, username: String, email: String, password: String) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        User {
            id,
            username,
            email,
            password,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_db(
        id: String,
        username: String,
        password: String,
        email: String,
        created_at: u64,
        updated_at: u64,
    ) -> Self {
        User {
            id,
            username,
            email,
            password,
            created_at,
            updated_at,
        }
    }
}


impl TryFrom<libsql::Row> for User {
    type Error = libsql::Error;

    fn try_from(row: libsql::Row) -> Result<Self, Self::Error> {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
            email: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}
