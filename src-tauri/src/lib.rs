use serde::{Deserialize, Serialize};
use std::{fmt, time::SystemTime};

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
pub struct Horse {
    id: String,
    name: String,
    breed: String,
    color: String,
    nationality: String,
    age: u32,
    gender: Gender,
    weight: u32,
    height: u32,
    length: u32,
}

#[derive(Deserialize)]
pub struct HorseCreate {
    pub name: String,
    pub breed: String,
    pub color: String,
    pub nationality: String,
    pub age: u32,
    pub gender: Gender,
    pub weight: u32,
    pub height: u32,
    pub length: u32,
}

impl Horse {

    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn breed(&self) -> &str {
        &self.breed
    }
    pub fn color(&self) -> &str {
        &self.color
    }
    pub fn nationality(&self) -> &str {
        &self.nationality
    }
    pub fn age(&self) -> u32 {
        self.age
    }
    pub fn gender(&self) -> &Gender {
        &self.gender
    }
    pub fn weight(&self) -> u32 {
        self.weight
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn length(&self) -> u32 {
        self.length
    }


}

impl TryFrom<libsql::Row> for Horse {
    type Error = libsql::Error;

    fn try_from(row: libsql::Row) -> Result<Self, Self::Error> {
        Ok(Horse {
            id: row.get(0)?,
            name: row.get(1)?,
            breed: row.get(2)?,
            color: row.get(3)?,
            nationality: row.get(4)?,
            age: row.get(5)?,
            gender: row.get::<String>(6)?.into(),
            weight: row.get(7)?,
            height: row.get(8)?,
            length: row.get(9)?,
        })
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

