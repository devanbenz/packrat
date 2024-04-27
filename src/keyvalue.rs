use std::fmt::Debug;
use tiny_bit_derive::{TinyBitDeserializer, TinyBitSerializer};

#[derive(TinyBitSerializer, TinyBitDeserializer, PartialEq, Debug)]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl KeyValue {
    pub fn new((key, value): (String, String)) -> Self {
        Self { key, value }
    }

    pub fn write_to_bytes(&self) -> Vec<u8> {
        KeyValue::serialize(self)
    }

    pub fn read_from_bytes(bytes: Vec<u8>) -> Vec<Self> {
        let mut kv_vec = Vec::new();
        let kvs = KeyValue::deserialize(bytes).expect("error deserializing");

        for kv in kvs {
            kv_vec.push(Self::new(kv));
        }

        kv_vec
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_new_kv() {
        let kv = KeyValue::new(("foo".to_string(), "bar".to_string()));
        assert_eq!(kv.value, "bar".to_string());
        assert_eq!(kv.key, "foo".to_string());
    }

    #[test]
    fn test_byte_serialization() {
        let kv = KeyValue::new(("foo".to_string(), "bar".to_string()));
        let bytes = kv.write_to_bytes();

        assert_eq!(bytes, vec![3, 102, 111, 111, 3, 98, 97, 114])
    }

    #[test]
    fn test_byte_deserialization() {
        let a = KeyValue::read_from_bytes(vec![
            3, 102, 111, 111, 3, 98, 97, 114, 3, 102, 111, 111, 3, 98, 97, 114, 3, 102, 111, 111,
            3, 98, 97, 114,
        ]);
        assert_eq!(a.len(), 3);
        assert_eq!(
            *a.first().unwrap(),
            KeyValue {
                key: "foo".to_string(),
                value: "bar".to_string(),
            }
        );
    }
}
