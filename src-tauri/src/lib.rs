use serde::{Deserialize, Serialize};
use std::{
    fmt, sync::atomic::{AtomicUsize, Ordering}
};

pub struct Stable {
    pub count: AtomicUsize,
}

impl Stable {
    pub fn new_horse(&self) -> u32 {
        self.count.fetch_add(1, Ordering::Relaxed) as u32
    }
}

#[derive(Debug, Serialize,Deserialize)]
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

impl From<String> for Gender {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Male" => Gender::Male,
            "Female" => Gender::Female,
            _ => Gender::Male,
        }
    }
}


#[derive(Debug, Serialize,Deserialize)]
pub struct Horse {
    pub id: u32,
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
    pub fn new(name: String, stable: &Stable) -> Self {
        Horse {
            id: stable.new_horse(),
            name,
            breed: String::from("Arabic"),
            color: String::from("red"),
            nationality: String::from("lebanese"),
            gender: Gender::Male,
            weight: 244,
            age: 10,
            height: 145,
            length: 250,
        }
    }
}
