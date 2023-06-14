use std::fmt::Debug;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct Storage {
    db: PickleDb,
}

impl Storage {
    pub fn new(file: &str) -> Storage {
        let db = match PickleDb::load(&file, PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin) {
            Ok(db) => db,
            Err(_) => PickleDb::new(file, PickleDbDumpPolicy::AutoDump, SerializationMethod::Bin),
        };

        Storage { db }
    }
}

impl Storage {
    pub fn set<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), String> {
        self.db.set(key, value).map_err(|e| e.to_string())
    }

    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        self.db.get::<T>(key)
    }

    pub fn remove(&mut self, key: &str) -> Result<bool, String> {
        self.db.rem(key).map_err(|e| e.to_string())
    }

    pub fn exists(&self, key: &str) -> bool {
        self.db.exists(key)
    }

    pub fn create_list(&mut self, name: &str) -> Result<(), String> {
        if !self.db.lexists(name) {
            return self.db.lcreate(name).map(|_| ()).map_err(|e| e.to_string());
        }
        Ok(())
    }

    pub fn ladd<T: Serialize + Debug>(&mut self, list: &str, value: &T) -> Result<(), String> {
        self.db
            .ladd(list, value)
            .ok_or(format!("Error on trying to persist object: {:?}", value))
            .map(|_| ())
    }

    pub fn lfirst<T: DeserializeOwned>(&self, list: &str) -> Option<T> {
        self.db.lget::<T>(list, 0)
    }

    pub fn lremove(&mut self, list: &str, i: usize) -> Result<(), String> {
        self.db.lpop::<()>(list, i).ok_or(format!(
            "Error on trying to remove object on position {} from list {}",
            i, list
        ))
    }

    pub fn list_is_empty(&self, list: &str) -> bool {
        self.db.llen(list) == 0
    }
}
