use serde_json::Value;
use crate::error::*;

pub mod ItemShop {
    use std::collections::HashMap;
    use crate::error::*;
    use crate::GenerateCommand;

    pub type ItemShopItems = HashMap<String, String>;
    pub type ItemShopCategories = HashMap<String, ItemShopItems>;

    pub struct ItemShopCommand {
        key: String,
        command: String,
        item: String,
    }

    impl ItemShopCommand {
        pub fn new(key: String, command: String, item: String) -> Self {
            Self { key, command, item }
        }
    }
    
    impl GenerateCommand for ItemShopCommand {
        fn generate(&self) -> Result<String> {
            if self.command.is_empty() {
                return Err(Error::new(ErrorKind::FieldEmpty, "ItemShopCommand's command field is empty".to_string()))
            }
            if self.item.is_empty() {
                return Err(Error::new(ErrorKind::FieldEmpty, "ItemShopCommand's item field is empty".to_string()))
            }
            if self.key.is_empty() {
                return Err(Error::new(ErrorKind::FieldEmpty, "ItemShopCommand's key field is empty".to_string()))
            }
            Ok(format!(r#"bind {} {} {}"#, self.key, self.command, self.item))
        }
    }
}

#[derive(Debug)]
pub enum ProgramJsonUsage {
    ItemShop {
        command: String,
        categories: ItemShop::ItemShopCategories,
    }
}

impl ProgramJsonUsage {
    pub fn parse_serde_value(usage_string: &str, value: Value) -> Result<Self> {
        match usage_string {
            "item_shop" => Self::parse_as_item_shop(value),
            s => return Err(ErrorKind::UsageInvalid.msg(format!("{} is not a valid usage", s))),
        }
    }

    pub fn parse_as_item_shop(mut value: Value) -> Result<Self> {
        use ItemShop::*;

        let command = match value["command"].take() {
            Value::String(s) => s,
            _ => return Err(ErrorKind::JsonInvalidType.msg("`command` have to be a string type".to_string())),
        };

        // Just convert serde map to hashmap
        // Looks like shit I know
        // I already forgot what I have done here
        let categories_serde = match value["categories"].take() {
            Value::Object(o) => o,
            _ => return Err(ErrorKind::JsonInvalidType.msg("`categories` has to be an object type".to_string())),
        };

        let mut categories = ItemShopCategories::new();
        for (ctg_name, items) in categories_serde.into_iter() {
            let items_serde = match items {
                    Value::Object(o) => o,
                    _ => return Err(ErrorKind::JsonInvalidType.msg("value of `categories` object has to be an object type, called `items`".to_string())),
            };

            let mut items = ItemShopItems::new();
            for (display, id) in items_serde.into_iter() {
                let id = match id {
                    Value::String(s) => s,
                    _ => return Err(ErrorKind::JsonInvalidType.msg("all of `items`'s value have to be a string type".to_string())),
                };
                
                items.insert(display, id);
            }

            categories.insert(ctg_name, items);
        }
        
        Ok(ProgramJsonUsage::ItemShop { command, categories })
    }
}

#[derive(Debug)]
pub struct Program {
    json_usage: ProgramJsonUsage,
}

impl Program {
    pub fn parse_serde_value(mut value: Value) -> Result<Self> {
        if !value.is_object() {
            return Err(ErrorKind::JsonInvalidType.msg("json file has to have object type first".to_string()))
        }
        

        let usage = match value["usage"].take() {
            Value::String(s) => s,
            _ => return Err(ErrorKind::JsonInvalidType.msg("".to_string())),
        };

        let pusage = ProgramJsonUsage::parse_serde_value(&usage, value);

        return match pusage {
            Ok(o) => Ok(Program { json_usage: o }),
            Err(e) => Err(e),
        }
    }

    pub fn item_shop_generate(&self) -> String {
        todo!()
    }
}
