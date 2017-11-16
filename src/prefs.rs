pub trait Key {
    type PreferenceValueType: Value;
    fn raw_key(&self) -> String;
}

pub trait Value: Sized {
    fn decode(data: &[u8]) -> Option<Self>;
    fn encode(&self) -> Option<Vec<u8>>;
}

pub trait Preferences: Send {
    fn get<Key: self::Key>(&self, key: Key) -> Option<Key::PreferenceValueType>;
    fn set<Key: self::Key<PreferenceValueType=Value>, Value: self::Value>(&self, value: Option<Value>, key: Key);
}
