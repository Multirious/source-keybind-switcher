use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader};
use serde_json::{Value};

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

type ItemShopItems = HashMap<String, String>;
type ItemShopCategories = HashMap<String, ItemShopItems>;

pub enum ProgramUiCfg {
    ItemShop {
        command: String,
        categories: ItemShopCategories,
    }
}

impl ProgramUiCfg {
    pub fn from_serde_value(cfg_string: &str, value: Value) -> Result<Self> {
        match cfg_string {
            "item_shop" => {
                let command = match value["command"] {
                    Value::String(s) => s,
                    _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
                };

                // Just convert serde map to hashmap
                // Maybe I should use .map()
                let categories = match value["categories"] {
                    Value::Object(o) => {
                        let categories = ItemShopCategories::new();
                        for (ctg_name, items) in o.into_iter() {
                            let items_serde = match items {
                                    Value::Object(o) => o,
                                    _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
                            };

                            let items = ItemShopItems::new();
                            for (display, id) in items_serde.into_iter() {
                                let id = match id {
                                    Value::String(s) => s,
                                    _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
                                };
                                
                                items.insert(display, id);
                            }

                            categories.insert(ctg_name, items);
                        }

                        categories
                    },
                    _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
                };
                
                Ok(ProgramUiCfg::ItemShop { command, categories })
            } 
        }
    }
}

pub struct Program {
    cfg: ProgramUiCfg,
}

impl Program {
    pub fn parse_serde_value(value: Value) -> Result<Self> {
        let map = match value {
            Value::Object(o) => o,
            _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
        };

        let cfg = match map["cfg"] {
            Value::String(s) => s,
            _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
        };

        let pcfg = ProgramUiCfg::from_serde_value(cfg, value);

        return Ok(())
    }
}

pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let val = serde_json::from_reader::<BufReader<File>, Value>(reader)?;

    Ok(val)
}
