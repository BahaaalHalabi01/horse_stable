use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Stable {
    pub count: AtomicUsize,
}

impl Stable {
    pub fn new_horse(&self) -> usize {
        self.count.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Debug, Serialize)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Debug, Serialize)]
pub struct Horse {
    pub id: usize,
    pub name: String,
    pub breed: String,
    pub color: String,
    pub nationality: String,
    pub age: usize,
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
