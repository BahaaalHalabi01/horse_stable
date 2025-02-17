use serde::{Deserialize, Serialize};
use std::fmt;

use crate::Gender;

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
    cleaness: u32,
    food: u32,
    water: u32,
    current_activity: Activity,
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
    pub fn cleaness(&self) -> u32 {
        self.cleaness
    }
    pub fn food(&self) -> u32 {
        self.food
    }
    pub fn water(&self) -> u32 {
        self.water
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
            cleaness: row.get(10)?,
            food: row.get(11)?,
            water: row.get(12)?,
            current_activity: row.get::<String>(13)?.into(),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Activity {
    Cleaning,
    Feeding,
    Watering,
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<String> for Activity {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Cleaning" => Activity::Cleaning,
            "Feeding" => Activity::Feeding,
            "Watering" => Activity::Watering,
            _ => Activity::Cleaning,
        }
    }
}

impl From<Activity> for String {
    fn from(val: Activity) -> Self {
        match val {
            Activity::Cleaning => "Cleaning".to_string(),
            Activity::Feeding => "Feeding".to_string(),
            Activity::Watering => "Watering".to_string(),
        }
    }
}
