extern crate self as struct_to_config;

use config::{ConfigBuilder, Source, ValueKind};
use config::builder::DefaultState;
use lazy_static::lazy_static;
use serde_json::{Value};
#[cfg(test)]
mod tests;


const COULD_NOT_SET_ITEM : &'static str = "Could not set default for key/value pair on config_builder.";
const NOT_SUPPORTED_TYPE : &'static str = "Serialized object was not one of supported types.";


enum Kind {
    String, Float, Boolean, Object, Nil
}

lazy_static! {
    static ref DEFAULT_VEC : Vec<Value> = Vec::<Value>::new();
}
fn get_value_kind_from_value(val: &Value, should_panic: bool) -> (ValueKind, Kind) {
    match val {
        Value::Null => {
            (ValueKind::Nil, Kind::Nil)
        }
        Value::String(f ) => {
            (ValueKind::String(f.to_string()), Kind::String)
        },
        Value::Number(f ) => {
            (ValueKind::Float(f.as_f64().unwrap()), Kind::Float)
        },
        Value::Bool(f ) => {
            (ValueKind::Boolean(*f), Kind::Boolean)
        },
        Value::Object(o) => {
            let values : Vec<ValueKind> = o.iter().map(|i| {
               return get_value_kind_from_value(i.1, should_panic).0.clone();
            }).collect();
            let val_kind = config::ValueKind::from(values);
            (val_kind, Kind::Object)
        },
        _ => {
            if should_panic {
                panic!("Found a value in array that is not a String, Number, Struct, or Bool");
            } else {
                (ValueKind::Nil, Kind::Nil)
            }
        }
    }
}
pub fn convert_panic(value : &Value, cfg_builder: Option<ConfigBuilder<DefaultState>>) -> ConfigBuilder<DefaultState> {
    let mut config_builder = cfg_builder.unwrap_or(config::Config::builder());
    if value.is_object() {
        let map = value.as_object();
        for item in map.unwrap() {
            if item.1.is_array() {
                let arr = item.1.as_array().expect(COULD_NOT_SET_ITEM);
                let mut new_arr : Vec<ValueKind> = Vec::new();
                for ref i in arr.iter() {
                    let val : ValueKind = get_value_kind_from_value(i, true).0;
                    new_arr.push(val);
                }
                config_builder = config_builder.set_default(item.0.as_str(), new_arr).expect(COULD_NOT_SET_ITEM);
            }else if item.1.is_string() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_str().unwrap()).expect(COULD_NOT_SET_ITEM);
            } else if item.1.is_i64() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_i64().unwrap()).expect(COULD_NOT_SET_ITEM);
            } else if item.1.is_boolean() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_bool().unwrap()).expect(COULD_NOT_SET_ITEM);
            } else if item.1.is_f64() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_f64().unwrap()).expect(COULD_NOT_SET_ITEM);
            } else if item.1.is_u64() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_u64().unwrap()).expect(COULD_NOT_SET_ITEM);
            } else if item.1.is_object() {
                config_builder = convert_panic(item.1, Some(config_builder));
            } else if item.1.is_null() {
                continue;
            } else {
                panic!("{}", NOT_SUPPORTED_TYPE);
            }
        }
    }
    config_builder
}

pub fn convert_non_panic(value : &Value, cfg_builder: Option<ConfigBuilder<DefaultState>>) -> ConfigBuilder<DefaultState> {
    let mut config_builder = cfg_builder.unwrap_or(config::Config::builder());
    if value.is_object() {
        let map = value.as_object();
        for item in map.unwrap() {
            if item.1.is_array() {
                let arr = item.1.as_array().unwrap_or(&DEFAULT_VEC);
                let mut new_arr : Vec<ValueKind> = Vec::new();
                for ref i in arr.iter() {
                    let val : ValueKind = get_value_kind_from_value(i, false).0;
                    new_arr.push(val);
                }
                config_builder = config_builder.set_default(item.0.as_str(), new_arr).unwrap_or_default();
            }else if item.1.is_string() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_str().unwrap()).unwrap_or_default();
            } else if item.1.is_i64() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_i64().unwrap()).unwrap_or_default();
            } else if item.1.is_boolean() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_bool().unwrap()).unwrap_or_default();
            } else if item.1.is_f64() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_f64().unwrap()).unwrap_or_default();
            } else if item.1.is_u64() {
                config_builder = config_builder.set_default(item.0.as_str(), item.1.as_u64().unwrap()).unwrap_or_default();
            } else if item.1.is_object() {
                config_builder = convert_non_panic(item.1, Some(config_builder));
            } else if item.1.is_null() {
                continue;
            } else {
                println!("Deserialized object is not one of supported types, so it was not added to the config.");
            }
        }
    }
    config_builder
}
#[macro_export]
/// Takes one or more identities of which are struct objects that implement serde Serializable.
/// Optionally, for the first argument, you may provide a boolean flag that if false, you would want the behavior of the macro to default on any struct attribute it cannot convert.
/// Otherwise, if true, it will panic.
/// If not providing a boolean literal as the first argument, you may list as many structs, with the last structs being of higher priority, i.e. it will override the prior key/value, if a matching key is found for its value.
/// Default behavior is that calling this macro will panic if a struct has a non-implemented type, such as (currently) an array of arrays.
macro_rules! make_cfg {
    ($($serializeable:ident),+) => {{
        use serde_json::Value;
        use struct_to_config::convert_panic;
        let mut config_builder = Config::builder();
        $(
        let value = serde_json::to_value($serializeable).unwrap();
        config_builder = convert_panic(&value, Some(config_builder));
        )+
        config_builder.build().unwrap()
    }};
    ($bool_should_panic:literal,$($serializeable:ident ),+) => {{
        use struct_to_config::convert_panic;
        use struct_to_config::convert_non_panic;
        let mut config_builder = Config::builder();
        if !$bool_should_panic {
            $(
            let value = serde_json::to_value($serializeable).unwrap();
            config_builder = convert_non_panic(&value, Some(config_builder));
            )+
        } else {
            $(
            let value = serde_json::to_value($serializeable).unwrap();
            config_builder = convert_panic(&value, Some(config_builder));
            )+
        }
        config_builder.build().unwrap()
    }};
}