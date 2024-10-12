# struct-to-config
## Purpose

This crate is to solve a problem regarding combining configs that are structs, easily, to my config sources.  I figured I could use Serde to serialize and add it manually this way.  This would have been tedious to do this for each use case, so I created a macro to do it.

### Features


1. Add clap structs to your config sources if you derive Serialize.
1. Aka., converts a struct that has serde::Serialize implemented to a Source-trait implemented Config struct from the config crate.
1. Two paths to handle structs that have non-implemented values: panic on run time, or skip.  
1. Within a single struct, nested attributes of structs take precedence if a key shares the same as a one closer to the root of the tree.  However, siblings are indeterministic.  For instance, an array with nested objects of which have an attribute that share the same as an attribute shared by the object containing the array would be inderministic.
1. Determine which attributes get priority (if they share the same key) by listing them in the make_cfg!() macro.
1. If make_cfg!(struct1, struct2) is called, than if an attribute from struct2's shares the same name as an attribute from struct1, struct2's attribute would take precedence over struct1's attribute (that shares the same name)

### Supported types

1. i64
2. f64
3. u64
4. string
5. Option<..>
6. objects (structs that implement Serialize)
7. bool
8. **SUPPORTS IN A LIMITED WAY** arrays with objects or a supported ValueKind enum from Serde crate (all numbers are automatically of the f64 type).  And, arrays do not support nested arrays at the moment.

#### Example that panics in runtime 

```rust
use config::Config;
use config::ConfigBuilder;
use serde::Serialize;
use clap::Parser;

#[derive(Serialize)]
#[clap(
    author = "Daniel Alexander Apatiga",
    version = "1.0.0",
    about = "Make service.")]
pub struct Make {
    /// Name of person
    #[clap(short, long)]
    name: String,
    good: Option<String>
}
pub fn test1() {
    let make = Make {
        name: "Joe".to_string(), 
        good: None
    };
    let mut cfg : Config = make_cfg!(make); //defaults to panic mode in runtime.
    assert_eq!(cfg.get_string("name").unwrap(), "Joe".to_string());
    let settings_builder = Config::builder()
        .add_source(config::File::with_name("path_to_file/blah.cfg"))
        .add_source(cfg);
    cfg = settings_builder.build().unwrap();
    assert_eq!(cfg.get_string("good").ok(), None);
}
```

#### Example that does not panic in runtime

```rust
use config::Config;
use config::ConfigBuilder;
use serde::Serialize;
use clap::Parser;

#[derive(Serialize)]
#[clap(
    author = "Daniel Alexander Apatiga",
    version = "1.0.0",
    about = "Make service.")]
pub struct Make {
    /// Name of person
    #[clap(short, long)]
    name: String,
    good: Option<String>
}
pub fn test1() {
    let make = Make {
        name: "Joe".to_string(), 
        good: None
    };
    let mut cfg : Config = make_cfg!(false, make); // Pass in false to skip unimplemented scenarios, such as arrays in arrays.
    assert_eq!(cfg.get_string("name").unwrap(), "Joe".to_string());
    let settings_builder = Config::builder()
        .add_source(config::File::with_name("path_to_file/blah.cfg"))
        .add_source(cfg);
    cfg = settings_builder.build().unwrap();
    assert_eq!(cfg.get_string("good").ok(), None);
}
```
