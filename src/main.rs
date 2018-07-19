extern crate rust_preferences;

use rust_preferences::prefs::*;
use rust_preferences::prefs::Preferences;
use rust_preferences::mem_prefs;

use std::thread;
use std::sync::{Arc};

use std::time::Duration;

struct NameKey {

}

impl NameKey {
    fn new() -> NameKey {
        NameKey{}
    }
}

#[derive(Debug)]
struct Name(String);

impl Key for NameKey {
    type PreferenceValueType = Name;

    fn raw_key(&self) -> String {
        String::from("Name Key")
    }
}

impl Value for Name {
    fn decode(data: &[u8]) -> Option<Self> {
        if let Ok(string) = String::from_utf8(Vec::from(data)) {
            Some(Name(string))
        }
        else {
            None
        }
    }

    fn encode(&self) -> Option<Vec<u8>> {
        Some(self.0.clone().into_bytes())
    }
}

fn main() {
    let prefs = Arc::new(mem_prefs::Preferences::new());
    let mut write_handles = Vec::new();
    for idx in 0..10 {
        let prefs = prefs.clone();
        let handle = thread::spawn(move || {
            prefs.set(Some(Name(format!("Alex {}", idx))), NameKey{});
        });
        write_handles.push(handle);
    }

    let mut read_handles = Vec::new();

    for _ in 0..10 {
        let prefs = prefs.clone();
        let handle = thread::spawn(move || {
            let name = prefs.get(NameKey::new());
            println!("Got: {:?}", name);
        });
        read_handles.push(handle);
    }
    std::thread::sleep(Duration::from_secs(10));
}
