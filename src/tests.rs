use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Make {
    name: String,
}

#[cfg(test)]
mod tests {
    use config::Config;
    use crate::make_cfg;
    use crate::tests::Make;

    #[test]
    pub fn test1() {
        let make = Make {
            name: "Randomstring".to_string()
        };
        let cfg = make_cfg!(make);
        assert_eq!(cfg.get_string("name").unwrap(), "Randomstring".to_string());
    }

}


