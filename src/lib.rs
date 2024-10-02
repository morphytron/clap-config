use config::{Source, ValueKind};
use serde_json::{value, Value};
mod tests;


enum Kind {
    String, Float, Boolean
}
fn get_value_kind_from_value(val: &Value) -> (ValueKind, Kind) {
    match val {
        Value::String(f ) => {
            (ValueKind::String(f.to_string()), Kind::String)
        },
        Value::Number(f ) => {
            (ValueKind::Float(f.as_f64().unwrap()), Kind::Float)
        },
        Value::Bool(f ) => {
            (ValueKind::Boolean(*f), Kind::Boolean)
        },
        _ => {
            panic!("Found a value in vec that is not a String, Number, or Bool");
            /*ValueKind::Nil*/
        }
    }
}

#[macro_export]
/// Takes one argument of which is a struct object (ident) that has implemented serde Deserializeable.
macro_rules! make_cfg {

    ($serializeable: ident) => {{
        use crate::get_value_kind_from_value;
        use config::ValueKind;
        let value = serde_json::to_value($serializeable).unwrap();
        use serde_json::Value;
        let mut config_builder = Config::builder();

        if value.is_object() {
            let map = value.as_object();
            for item in map.unwrap() {
                if item.1.is_array() {
                    let arr = item.1.as_array();
                    let new_arr  : Vec<ValueKind> = arr.iter().map(|i| get_value_kind_from_value(&i).0).collect();
                    config_builder = config_builder.set_default(item.0.as_str(), new_arr).expect("Could not set default on config_builder.  Result was an error.");
                }else if item.1.is_string() {
                    config_builder = config_builder.set_default(item.0.as_str(), item.1.as_str().unwrap()).expect("Could not set default on config_builder.  Result was an error.");
                } else if item.1.is_i64() {
                  config_builder = config_builder.set_default(item.0.as_str(), item.1.as_i64().unwrap()).expect("Could not set default on config_builder.  Result was an error.");
                } else if item.1.is_boolean() {
                    config_builder = config_builder.set_default(item.0.as_str(), item.1.as_bool().unwrap()).expect("Could not set default on config_builder.  Result was an error.");
                } else if item.1.is_f64() {
                   config_builder = config_builder.set_default(item.0.as_str(), item.1.as_f64().unwrap()).expect("Could not set default on config_builder.  Result was an error.");
                } else if item.1.is_u64() {
                  config_builder = config_builder.set_default(item.0.as_str(), item.1.as_u64().unwrap()).expect("Could not set default on config_builder.  Result was an error.");
                } else {
                    panic!("Deserialized object is not caught and added to config.");
                }
            }
        }
        config_builder.build().unwrap()
    }}
}