use prefs::*;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct MemoryPreferences {
    values: Mutex<HashMap<String, Vec<u8>>>
}

impl MemoryPreferences {
    pub fn new() -> MemoryPreferences {
        return MemoryPreferences { values: Mutex::new(HashMap::new()) }
    }
}

impl Preferences for MemoryPreferences {
    fn get<Key: PreferenceKey>(&self, key: Key) -> Option<Key::PreferenceValueType> {
        println!("getting '{}'", key.raw_key());
        let values = self.values.lock().unwrap();
        if let Some(value_data) = values.get(&key.raw_key()) {
            Key::PreferenceValueType::decode(&value_data)
        } else {
            None
        }
    }

    fn set<Key: PreferenceKey<PreferenceValueType=Value>, Value: PreferenceValue>(&self, value: Option<Value>, key: Key) {
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