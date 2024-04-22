use crate::keyvalue::KeyValue;

mod keyvalue;
mod wal;

fn main() {
    let kv = KeyValue::new(("foo".to_string(), "bar".to_string()));
    kv.write_to_bytes();
}
