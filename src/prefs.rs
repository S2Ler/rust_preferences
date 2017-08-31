//public protocol Preferences {
//func get<Key: PreferenceKey>(_ key: Key) -> Key.PreferenceValueType?
//func set<Key: PreferenceKey, Value>(_ value: Value?, for key: Key) where Key.PreferenceValueType == Value
//}
//
//public protocol PreferenceKey {
//associatedtype PreferenceValueType: PreferenceValue
//var rawKey: String { get }
//}
//
//public protocol PreferenceValue {
//static func decode(_ data: Data) -> Self?
//func encode() -> Data?
//}

pub trait PreferenceKey {
    type PreferenceValueType: PreferenceValue;
    fn raw_key(&self) -> String;
}

pub trait PreferenceValue: Sized {
    fn decode(data: &[u8]) -> Option<Self>;
    fn encode(&self) -> Option<Vec<u8>>;
}

pub trait Preferences: Send {
    fn get<Key: PreferenceKey>(&self, key: Key) -> Option<Key::PreferenceValueType>;
    fn set<Key: PreferenceKey<PreferenceValueType=Value>, Value: PreferenceValue>(&self, value: Option<Value>, key: Key);
}
