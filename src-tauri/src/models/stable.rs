use std::time::SystemTime;
use fake::faker::name::en::*;
use fake::faker::address::en::*;
use serde::{Deserialize, Serialize};

use fake::Dummy;

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

#[derive(Deserialize,Dummy,Debug)]
pub struct StableCreate {
    #[dummy(faker = "Name()")]
    pub name: String,
    #[dummy(faker = "CityName()")]
    pub address: String,
    #[dummy(faker = "100..1000")]
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

