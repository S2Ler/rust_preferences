use std::collections::HashMap;
use std::sync::Mutex;
use prefs;
use prefs::{Value};

pub struct Preferences {
    values: Mutex<HashMap<String, Vec<u8>>>
}

impl Preferences {
    pub fn new() -> Preferences {
        return Preferences { values: Mutex::new(HashMap::new()) }
    }
}

impl prefs::Preferences for Preferences {
    fn get<Key: prefs::Key>(&self, key: Key) -> Option<Key::PreferenceValueType> {
        println!("getting '{}'", key.raw_key());
        let values = self.values.lock().unwrap();
        if let Some(value_data) = values.get(&key.raw_key()) {
            Key::PreferenceValueType::decode(&value_data)
        } else {
            None
        }
    }

    fn set<Key: prefs::Key<PreferenceValueType=Value>, Value: prefs::Value>(&self, value: Option<Value>, key: Key) {
        let key = key.raw_key();
        println!("setting '{}'", key);
        let mut values = self.values.lock().unwrap();
        if let Some(Some(value_data)) = value.map(|v| v.encode()) {
            values.insert(key, value_data);
        }
        else {
            values.remove(&key);
        }
    }
}