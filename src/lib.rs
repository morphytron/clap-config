use config::{ Source };
use serde_json::value;
mod tests;

#[macro_export]
/// Takes one argument of which is a struct object (ident) that has implemented serde Deserializeable.
macro_rules! make_cfg {
    ($serializeable: ident) => {{
        let value = serde_json::to_value($serializeable).unwrap();
        let mut config_builder = Config::builder();

        if value.is_object() {
            let map = value.as_object();
            for item in map.unwrap() {
                if item.1.is_string() {
                    config_builder = config_builder.set_default(item.0.as_str(), item.1.as_str().unwrap()).expect("Could not set default on config_builder.  Result was an error.");
                } else if item.1.is_i64() {
                  config_builder = config_builder.set_default(item.0.as_str(), item.1.as_i64().unwrap()).expect("Could not set default on config_builder.  Result was an error.");
                }else if item.1.is_boolean() {
                    config_builder = config_builder.set_default(item.0.as_str(), item.1.as_bool().unwrap()).expect("Could not set default on config_builder.  Result was an error.");
                }else if item.1.is_f64() {
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