# struct-to-config
## Purpose

I ran into a problem with adding clap structs into a config file, easily, of which I figured I could use Serde to serialize and add it manually that way.  This was tedious to do this for each use case, so I designed this crate to be simple to use.

### Features

Fails on compile time if any attribute of a struct (doesn't necessarily have to be a Clap arg struct) does not convert into a key-value pair given the current narrow implementation.

1. Add clap structs to your config sources if you derive Serialize.
2. Convert any struct to a Source-implemented Config struct from the config crate.
3. Within a single struct, nested attributes of structs take precedence if a key shares the same as a one closer to the root of the tree.  However, siblings are indeterministic.  For instance, an array with nested objects of which have an attribute that share the same as an attribute shared by the object containing the array would be inderministic.
4. Determine which attributes get priority (if they share the same key) by listing them in the make_cfg!() macro.
5. If make_cfg!(struct1, struct2) is called, then struct2's attributes would take precedence over struct1's attributes if they share the same identifier.
### Supported types

1. i64
2. f64
3. u64
4. string
5. Option<..>
6. objects (structs that implement Serialize)
7. bool
8. **SUPPORTS IN A LIMITED WAY** arrays with objects or a supported ValueKind enum from Serde crate (all numbers are automatically of the f64 type).  And, arrays do not support nested arrays at the moment.

Example

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
    let mut cfg : Config = make_cfg!(make);
    assert_eq!(cfg.get_string("name").unwrap(), "Joe".to_string());
    let settings_builder = settings_builder
        .add_source(config::File::with_name("path_to_file/blah.cfg"))
        .add_source(cfg);
    cfg = settings_builder.build().unwrap();
    assert_eq!(cfg.get_string("joe").ok(), None);
}
```
