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

pub struct Item {
    display: String,
    id: String,
}

impl Item {
    pub fn new(display: String, id: String) -> Self {
        Item { display, id }
    }
}

pub struct Category {
    name: String,
    items: Vec<Item>,
}

impl Category {
    pub fn new(name: String, items: Vec<Item>) -> Self {
        Category { name, items }
    }
}

pub struct ItemCategories {
    categories: Vec<Category>,
}

impl ItemCategories {
    pub fn new(categories: Vec<Category>) -> Self {
        ItemCategories { categories }
    }
}

impl ItemCategories {
    // pub fn from_serde_value(value: Value) -> Result<Self> {
    //     let map = match value {
    //         Value::Object(o) => o,
    //         _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
    //     };
        
    //     let categories: Vec<Category>
    //     for (i, (ctg_name, items)) in map.into_iter().enumerate() {
    //         let items_o = match items {
    //             Value::Object(o) => o,
    //             _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string()))
    //         };


            
    //         let ctg = Category::new(ctg_name, vec![]);
    //         categories.push(ctg);
    //     }

    //     return Ok(())
    // }
}

pub fn load_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let val = serde_json::from_reader::<BufReader<File>, Value>(reader)?;

    Ok(val)
}
