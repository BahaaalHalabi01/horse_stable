use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

mod models;

pub use models::*;

//same order stored in the db
#[derive(Debug, Serialize, Deserialize)]
pub struct Stable {
    id: u32,
    name: String,
    address: String,
    monthly_fee: u32,
    created_at: u64,
    updated_at: u64,
}

#[derive(Deserialize)]
pub struct StableCreate {
    pub name: String,
    pub address: String,
    pub monthly_fee: u32,
}

impl Stable {
    pub fn new(id: u32, name: String, address: String, monthly_fee: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Stable {
            id,
            name,
            address,
            monthly_fee,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn address(&self) -> String {
        self.address.clone()
    }
    pub fn monthly_fee(&self) -> u32 {
        self.monthly_fee
    }
    pub fn created_at(&self) -> u64 {
        self.created_at
    }
    pub fn updated_at(&self) -> u64 {
        self.updated_at
    }
}

impl TryFrom<libsql::Row> for Stable {
    type Error = libsql::Error;

    fn try_from(row: libsql::Row) -> Result<Self, Self::Error> {
        Ok(Stable {
            id: row.get(0)?,
            name: row.get(1)?,
            address: row.get(2)?,
            monthly_fee: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

