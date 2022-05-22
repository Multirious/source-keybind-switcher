pub mod error {
    use std::result;
    use std::io;

    use crate::utils::*;
    
    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum ErrorKind {
        SerdeJson(serde_json::Error),
        IO(io::Error),
        JsonInvalidType,
        UsageInvalid,
        CommandSetLenUnder2,
        FieldEmpty,
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

pub trait GenerateCommand {
    fn generate(&self) -> error::Result<String>;
}

pub mod keybind_switcher {
    use crate::utils::{error::{*, self}, GenerateCommand};
    use serde::{Serialize, Deserialize};

    #[cfg(test)]
    mod test {
        use crate::utils::{keybind_switcher::{KeybindSwitcher, CommandSet}, GenerateCommand};

        #[test]
        fn test() {
            fn s(str: &str) -> String {
                str.to_string()
            }

            let cmds = vec![
                CommandSet::new(s("first"), s("third"), s("second"), vec![s("echo kill"), s("echo sex")]),
                CommandSet::new(s("second"), s("first"), s("third"), vec![s("echo blowjob"), s("echo givehead")]),
                CommandSet::new(s("third"), s("second"), s("first"), vec![s("echo drink"), s("echo eat")]),
            ];

            let kbsw = KeybindSwitcher::new(s("test"), s("pgup"), s("pgdn"), cmds);

            let g = kbsw.generate().unwrap();

            println!("{}", g);
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct KeybindSwitcher {
        name: String,
        key_next: String,
        key_previous: String,
        command_sets: Vec<CommandSet>
    }

    impl KeybindSwitcher {
        pub fn new(name: String, key_next: String, key_previous: String, command_sets: Vec<CommandSet>) -> Self {
            Self { name, key_next, key_previous, command_sets }
        }
    }
    
    impl GenerateCommand for KeybindSwitcher {
        fn generate(&self) -> Result<String> {
            fn add_line(s: &mut String, cmd: &str) {
                s.push_str(cmd);
                s.push('\n');
            }

            if self.command_sets.len() < 2 {
                return Err(Error::new(ErrorKind::CommandSetLenUnder2, "Command Set have to be more than 2 for this to works properly".to_string()))
            }
            if self.name.is_empty() {
                return Err(Error::new(ErrorKind::FieldEmpty, "KeybindSwitcher's name field is empty".to_string()))
            }
            if self.key_next.is_empty() {
                return Err(Error::new(ErrorKind::FieldEmpty, "KeybindSwitcher's key_next field is empty".to_string()))
            }
            if self.key_previous.is_empty() {
                return Err(Error::new(ErrorKind::FieldEmpty, "KeybindSwitcher's key_previous field is empty".to_string()))
            }


            let mut switcher_init_cmd = String::new();
            let mut s = String::new();
            add_line(&mut s, &format!("//Keybind Switcher {}", self.name));
            
            for (i, cs) in self.command_sets.iter().enumerate() {
                if cs.name.is_empty() {
                    return Err(Error::new(ErrorKind::FieldEmpty, "CommandSet's name field is empty".to_string()))
                }
                if cs.next_set.is_empty() {
                    return Err(Error::new(ErrorKind::FieldEmpty, "CommandSet's next_set field is empty".to_string()))
                }
                if cs.previous_set.is_empty() {
                    return Err(Error::new(ErrorKind::FieldEmpty, "CommandSet's previous_set field is empty".to_string()))
                }

                let alias_next = &cs.next_set;
                let alias_prev = &cs.previous_set;
                let alias_curr = format!("_sw_{}", cs.name);

                add_line(&mut s, &format!(r#"alias "{}" "bind {} _sw_{};bind {} _sw_{};{}_cmds""#,
                    alias_curr,
                    self.key_next,
                    alias_next,
                    self.key_previous,
                    alias_prev,
                    alias_curr,
                ));

                let mut cmds = String::new();
                cmds.push_str(&format!(r#"alias "{}_cmds" "echo Switched {}.{};"#, alias_curr, self.name, cs.name));
                
                for (i, c) in cs.commands.iter().enumerate() {
                    let name = format!("{}_cmd{}", alias_curr, i);
                    add_line(&mut s, &format!(r#"alias "{}" {}"#, name, c));
                    cmds.push_str(&name);
                    cmds.push(';');
                }
                cmds.push('"');
                add_line(&mut s, &cmds);
                s.push('\n');

                if i == 0 {
                    switcher_init_cmd = alias_curr;
                }
            }

            s.push_str(&switcher_init_cmd);
            s.push('\n');

            return Ok(s)
        }
    }

    #[derive(Serialize, Deserialize)]
    pub struct CommandSet {
        name: String,
        previous_set: String,
        next_set: String,
        commands: Vec<String>,
    }

    impl CommandSet {
        pub fn new(name: String, previous_alias: String, next_set: String, commands: Vec<String>) -> Self {
            Self { name, previous_set: previous_alias, next_set, commands }
        }
    }
}

pub mod program {
    use serde_json::Value;
    use crate::utils::error::*;
    
    pub mod ItemShop {
        use std::collections::HashMap;
        use crate::utils::error::*;
        use crate::utils::GenerateCommand;

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
                s => return Err(Error::new(ErrorKind::UsageInvalid, format!("{} is not a valid usage", s))),
            }
        }

        pub fn parse_as_item_shop(mut value: Value) -> Result<Self> {
            use ItemShop::*;

            let command = match value["command"].take() {
                Value::String(s) => s,
                _ => return Err(Error::new(ErrorKind::JsonInvalidType, "`command` have to be a string type".to_string())),
            };

            // Just convert serde map to hashmap
            // Looks like shit I know
            // I already forgot what I have done here
            let categories = match value["categories"].take() {
                Value::Object(o) => {
                    let mut categories = ItemShopCategories::new();
                    for (ctg_name, items) in o.into_iter() {
                        let items_serde = match items {
                                Value::Object(o) => o,
                                _ => return Err(Error::new(ErrorKind::JsonInvalidType, "value of `categories` object has to be an object type, called `items`".to_string())),
                        };

                        let mut items = ItemShopItems::new();
                        for (display, id) in items_serde.into_iter() {
                            let id = match id {
                                Value::String(s) => s,
                                _ => return Err(Error::new(ErrorKind::JsonInvalidType, "all of `items`'s value have to be a string type".to_string())),
                            };
                            
                            items.insert(display, id);
                        }

                        categories.insert(ctg_name, items);
                    }

                    categories
                },
                _ => return Err(Error::new(ErrorKind::JsonInvalidType, "`categories` has to be an object type".to_string())),
            };
            
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
                return Err(Error::new(ErrorKind::JsonInvalidType, "json file has to have object type first".to_string()))
            }
            
    
            let usage = match value["usage"].take() {
                Value::String(s) => s,
                _ => return Err(Error::new(ErrorKind::JsonInvalidType, "".to_string())),
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
}

mod data{
    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;
    use serde_json::Value;
    use crate::utils::error::*;
    use crate::utils::keybind_switcher::KeybindSwitcher;

    pub fn import_json<P: AsRef<Path>>(path: P) -> Result<Value> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
    
        let val = serde_json::from_reader::<BufReader<File>, Value>(reader)?;
    
        Ok(val)
    }

    pub fn export_switcher<P: AsRef<Path>>(path: P, kbsw: KeybindSwitcher) -> Result<()> {
        todo!()
    }

    pub fn import_switcher<P: AsRef<Path>>(path: P) -> Result<KeybindSwitcher> {
        todo!() 
    }
}
