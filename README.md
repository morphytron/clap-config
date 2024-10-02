# clap-config
## Purpose

I ran into a problem with adding clap structs into a config file, easily, of which I figured I could use Serde to deserialize and add it manually that way.  This was tedious to do this for each use case, so I designed this crate to be simple to use.

### Features

Fails on compile time if any attribute of a struct (doesn't necessarily have to be a Clap arg struct) does not convert into a key-value pair given the current narrow implementation.

1. Add clap structs to your config sources.
2. Convert any struct to a Source-implemented Config struct from the config crate.

### Supported types

1. i64
2. f64
3. u64
4. string
5. bool
6. **DOES NOT** support arrays yet.

Example

```rust
use config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Make {
    name: String,
}
pub fn test1() {
    let make = Make {
        name: "Randomstring".to_string()
    };
    let cfg = make_cfg!(make);
    assert_eq!(cfg.get_string("name").unwrap(), "Randomstring".to_string());
}
```
