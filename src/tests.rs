use serde::{ Serialize};
use config::Config;
use crate::make_cfg;
#[derive(Serialize,Debug, Clone)]
pub struct Make {
    name: String,
}

#[derive(Serialize,Debug)]
pub struct MakeWithArray {
    name: String,
    number: i16,
    array: Vec<String>
}
#[derive(Serialize,Debug, Clone)]
pub struct MakeWithArrayAndObject {
    name: String,
    number: i16,
    array: Vec<String>,
    other_make: Make
}
#[derive(Serialize,Debug, Clone)]
pub struct MakeWithArrayOfObjects {
    name: String,
    number: i16,
    array: Vec<Make>,
}

#[derive(Serialize,Debug, Clone)]
pub struct MakeWithOption {
    name: Option<String>,
}

#[test]
pub fn test1() {
    let make = Make {
        name: "Randomstring".to_string()
    };
    let cfg = make_cfg!(make);
    assert_eq!(cfg.get_string("name").unwrap(), "Randomstring".to_string());
}

#[test]
pub fn test_struct_with_array() {
    let make = MakeWithArray {
        name: "Randomstring".to_string(),
        number: 32i16,
        array: vec!["Randomstring".to_string(), "blah".to_string()]
    };
    let cfg = make_cfg!(make);
}
#[test]
pub fn test_struct_with_array_with_object() {
    let make = MakeWithArrayAndObject {
        name: "Randomstring".to_string(),
        number: 32i16,
        array: vec!["Randomstring".to_string(), "blah".to_string()],
        other_make: Make {
            name: "Overriding attribute".to_string(),
        }
    };
    let cfg = make_cfg!(true, make);
    assert_eq!(cfg.get_string("name").unwrap(), "Overriding attribute".to_string());
}
#[test]
pub fn test_struct_with_array_of_objects() {
    let basic_make = Make {
        name: "Overriding attribute".to_string(),
    };
    let basic_make2 =Make {
        name: "Overriding attribute 3".to_string(),
    };
    let make = MakeWithArrayOfObjects {
        array: vec!(basic_make, basic_make2),
        name: "Randomstring".to_string(),
        number: 32i16,
    };
    let cfg = make_cfg!(true, make);
    assert_eq!(cfg.get_int("number").unwrap(), 32i64);
}

#[test]
pub fn test_option_string() {
    let make_basic = MakeWithOption {
        name: None
    };
    let cfg = make_cfg!(make_basic);

    assert_eq!(cfg.get_string("name").ok(), None);
}


