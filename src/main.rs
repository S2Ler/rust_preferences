mod prefs;
mod mem_prefs;

use prefs::*;
use mem_prefs::*;
use prefs::Preferences;

use std::thread;
use std::sync::{Arc};

struct NameKey {

}

impl NameKey {
    fn new() -> NameKey {
        NameKey{}
    }
}

#[derive(Debug)]
struct Name(String);

impl PreferenceKey for NameKey {
    type PreferenceValueType = Name;

    fn raw_key(&self) -> String {
        String::from("Name Key")
    }
}

impl PreferenceValue for Name {
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
    let prefs = Arc::new(MemoryPreferences::new());
    let mut handles = Vec::new();
    for idx in 0..10 {
        let prefs = prefs.clone();
        let handle = thread::spawn(move || {
            prefs.set(Some(Name(format!("Alex {}", idx))), NameKey{});
        });
        handles.push(handle);
    }

    let mut handles2 = Vec::new();

    for _ in 0..10 {
        let prefs = prefs.clone();
        let handle = thread::spawn(move || {
            let name = prefs.get(NameKey::new());
            println!("Got: {:?}", name);
        });
        handles2.push(handle);
    }

    thread::sleep_ms(10000000);

//    let prefs = Arc::new(Mutex::new(MemoryPreferences::new()));
//    let mut handles2 = Vec::new();
//    for _ in 0..10 {
//        let prefs = prefs.clone();
//        let handle = thread::spawn(move || {
//            let mut a = prefs.lock().unwrap();
//            a.set(Some(Name(String::from("Alex"))), NameKey{});
//        });
//        handles2.push(handle);
//    }
//
//    for handle in handles2 {
//        handle.join().unwrap();
//    }

}
