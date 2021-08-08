extern crate toml;
extern crate serde_derive;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

use serde::de::DeserializeOwned;

/// Loads TOML file into TOML object, returns either TOML object or Error
pub fn load_toml<T: DeserializeOwned>(filename: &str) -> Result<T, Box<dyn Error>> {
    let filepath: &str = &format!("{}", filename);

    let mut filehdlr = match File::open(filepath) {
        Ok(f) => f,
        Err(e) => panic!("Error occurred opening file: {} - Err: {}", filepath, e)
    };

    let mut filecont = String::new();
    match filehdlr.read_to_string(&mut filecont) {
          Ok(s) => s
        , Err(e) => panic!("Error Reading file: {}", e)
    };

    let s: T = toml::from_str(&filecont)?;

    Ok(s)
}
