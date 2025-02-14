use serde::{Deserialize, Serialize};
use std::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
    time::SystemTime,
};

//same order stored in the db
pub struct Stable {
    id: u32,
    name: String,
    address: String,
    monthly_fee: u32,
    created_at: u64,
    updated_at: u64,
    horse_count: AtomicU64,
}

impl Stable {
    pub fn new(id: u32, name: String, address: String, monthly_fee: u32, horse_count: u64) -> Self {
        let horse_count = AtomicU64::new(horse_count);
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
            horse_count,
        }
    }

    pub fn from_db(
        id: u32,
        name: String,
        address: String,
        monthly_fee: u32,
        created_at: u64,
        updated_at: u64,
        horse_count: u64,
    ) -> Self {
        let horse_count = AtomicU64::new(horse_count);
        Stable {
            id,
            name,
            address,
            monthly_fee,
            created_at,
            updated_at,
            horse_count,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
    pub fn horse_count(self) -> AtomicU64 {
        self.horse_count
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

    pub fn new_horse(&self) -> u32 {
        self.horse_count.fetch_add(1, Ordering::Relaxed) as u32
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
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
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

impl Horse {
    // pub fn new(name: String, stable: &Stable) -> Self {
    //     Horse {
    //         id: stable.new_horse(),
    //         name,
    //         breed: String::from("Arabic"),
    //         color: String::from("red"),
    //         nationality: String::from("lebanese"),
    //         gender: Gender::Male,
    //         weight: 244,
    //         age: 10,
    //         height: 145,
    //         length: 250,
    //     }
    // }
    pub fn from_db(
        id: String,
        name: String,
        breed: String,
        color: String,
        nationality: String,
        age: u32,
        gender: String,
        weight: u32,
        height: u32,
        length: u32,
    ) -> Self {
        Horse {
            id,
            name,
            breed,
            color,
            nationality,
            gender: gender.into(),
            weight,
            age,
            height,
            length,
        }
    }

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
