use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader};
use serde_json::{Value, Map};

use error::*;

mod error {
    use std::result;

    use super::*;
    
    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum ErrorKind {
        SerdeJson(serde_json::Error),
        IO(io::Error),
        JsonImportError,
    }
    
    #[derive(Debug)]
    pub struct Error {
        kind: ErrorKind,
        msg: String,
    }
    
    impl Error {
        pub fn new(kind: ErrorKind, msg: String) -> Self {
            Error { kind, msg }
        }

        pub fn kind(kind: ErrorKind) -> Self {
            Self::new(kind, "".to_string())
        }
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self)
        }
    }
    
    impl std::error::Error for Error {}
    
    impl From<serde_json::Error> for Error {
        fn from(e: serde_json::Error) -> Self {
            Error::new(ErrorKind::SerdeJson(e), "".to_string())
        }
    }
    
    impl From<io::Error> for Error {
        fn from(e: io::Error) -> Self {
            Error::new(ErrorKind::IO(e), "".to_string())
        }
    }
}

/// Key is display name and value is the ID
type ShopItemItems = HashMap<String, String>;
/// Key is category name and key is Items
type ShopItemArgList = HashMap<String, ShopItemItems>;

pub enum CommandUIType {
    ShopItem {
        command: String,
        arg_list: ShopItemArgList,
    },
}
// zs_purchaseitems peashooter; zs_purchaseitems unhinger
struct Program {
    command_ui_type: CommandUIType,
}

impl Program {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self> {
        let value = load_json(path);

        let ctgs_serde_map = match value {
            Value::Object(o) => o,
            _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
        };
    
        let mut ctgs_hash_map = ShopItemArgList::new();
        for (ctg_name, items) in ctgs_serde_map.into_iter() {
            let items_serde_map =  match items {
                Value::Object(o) => o,
                _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
            };
            
            let mut items_hash_map = ShopItemItems::new();
            for (display, id) in items_serde_map.into_iter() {
                let id = match id {
                    Value::String(s) => s,
                    _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
                };
                items_hash_map.insert(display, id);
            }
    
            ctgs_hash_map.insert(ctg_name, items_hash_map);
        }
    
        return Ok(ctgs_hash_map)
    }
}

pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let val = serde_json::from_reader::<BufReader<File>, Value>(reader)?;

    Ok(val)
}
