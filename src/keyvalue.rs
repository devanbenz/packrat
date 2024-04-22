use std::fmt::Debug;
use tiny_bit_derive::{TinyBitDeserializer, TinyBitSerializer};

#[derive(TinyBitSerializer, Debug, TinyBitDeserializer)]
pub struct KeyValue<T> {
    pub key: String,
    pub value: T,
}

impl<T> KeyValue<T>
where
    T: Debug,
{
    pub fn new((key, value): (String, T)) -> Self {
        Self { key, value }
    }

    pub fn write_to_bytes(&self) -> Vec<u8> {
        KeyValue::serialize(self)
    }

    pub fn read_from_bytes(&self, bytes: Vec<u8>) -> () {
        KeyValue::deserialize(bytes);
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
        let kv = KeyValue::new(("foo".to_string(), 100));
        assert_eq!(
            kv.write_to_bytes(),
            vec![
                75, 101, 121, 86, 97, 108, 117, 101, 32, 123, 32, 107, 101, 121, 58, 32, 34, 102,
                111, 111, 34, 44, 32, 118, 97, 108, 117, 101, 58, 32, 49, 48, 48, 32, 125
            ]
        )
    }

    #[test]
    fn test_byte_deserialization() {}
}
