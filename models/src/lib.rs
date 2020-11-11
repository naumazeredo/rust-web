use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicU32;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Clone)]
pub struct Database {
    pub new_id: Arc<AtomicU32>,
    pub patients : Arc<RwLock<HashMap<u32, Patient>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            new_id : Arc::new(AtomicU32::new(1)),
            patients : Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Patient {
    pub id: u32,
    pub name: String,
    pub age: i32,
}
