mod error {
    use std::result;
    use std::io;

    use super::*;
    
    pub type Result<T> = result::Result<T, Error>;

    #[derive(Debug)]
    pub enum ErrorKind {
        SerdeJson(serde_json::Error),
        IO(io::Error),
        JsonImportError,
        CfgTypeNotValid,
        CommandSetUnder2,
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

mod command_generator {
    use super::error::*;
    use serde::{Serialize, Deserialize};

    #[cfg(test)]
    mod test {
        use crate::utils::command_generator::{KeybindSwitcher, CommandSet};

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

        pub fn generate(&self) -> Result<String> {
            fn add_line(s: &mut String, cmd: &str) {
                s.push_str(cmd);
                s.push('\n');
            }

            if self.command_sets.len() < 2 {
                return Err(Error::new(ErrorKind::CommandSetUnder2, "Command Set have to be more than 2 for this to works properly".to_string()))
            }

            let mut switcher_init_cmd = String::new();
            let mut s = String::new();
            add_line(&mut s, &format!("//Keybind Switcher {}", self.name));
            
            for (i, cs) in self.command_sets.iter().enumerate() {
                let alias_next = &cs.next_alias;
                let alias_prev = &cs.previous_alias;
                let alias_curr = format!("_sw_{}", cs.alias_name);
                let alias_bind_next = format!("{}_bind_key_next", alias_curr);
                let alias_bind_prev = format!("{}_bind_key_prev", alias_curr);

                add_line(&mut s, &format!(r#"alias "{}" "bind {} _sw_{};bind {} _sw_{};{}_cmds""#,
                    alias_curr,
                    self.key_next,
                    alias_next,
                    self.key_next,
                    alias_next,
                    alias_curr,
                ));

                let mut cmds = String::new();
                cmds.push_str(&format!(r#"alias "{}_cmds" "echo Switched {}.{};"#, alias_curr, self.name, cs.alias_name));
                
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
        alias_name: String,
        previous_alias: String,
        next_alias: String,
        commands: Vec<String>,
    }

    impl CommandSet {
        pub fn new(alias_name: String, previous_alias: String, next_alias: String, commands: Vec<String>) -> Self {
            Self { alias_name, previous_alias, next_alias, commands }
        }
    }
}

mod program {
    use serde_json::Value;
    use std::collections::HashMap;
    use super::error::*;

    type ItemShopItems = HashMap<String, String>;
    type ItemShopCategories = HashMap<String, ItemShopItems>;
    
    #[derive(Debug)]
    pub enum ProgramUiCfg {
        ItemShop {
            command: String,
            categories: ItemShopCategories,
        }
    }
    
    impl ProgramUiCfg {
        pub fn from_serde_value(cfg_string: &str, value: Value) -> Result<Self> {
            match cfg_string {
                "item_shop" => Self::parse_as_item_shop(value),
                s => return Err(Error::new(ErrorKind::CfgTypeNotValid, format!("{} is not a valid cfg type", s))),
            }
        }

        pub fn parse_as_item_shop(mut value: Value) -> Result<Self> {
            let command = match value["command"].take() {
                Value::String(s) => s,
                _ => return Err(Error::new(ErrorKind::JsonImportError, "`command` have to be a string type".to_string())),
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
                                _ => return Err(Error::new(ErrorKind::JsonImportError, "value of `categories` object has to be an object type, called `items`".to_string())),
                        };

                        let mut items = ItemShopItems::new();
                        for (display, id) in items_serde.into_iter() {
                            let id = match id {
                                Value::String(s) => s,
                                _ => return Err(Error::new(ErrorKind::JsonImportError, "all of `items`'s value have to be a string type".to_string())),
                            };
                            
                            items.insert(display, id);
                        }

                        categories.insert(ctg_name, items);
                    }

                    categories
                },
                _ => return Err(Error::new(ErrorKind::JsonImportError, "`categories` has to be an object type".to_string())),
            };
            
            Ok(ProgramUiCfg::ItemShop { command, categories })
        }
    }
    
    #[derive(Debug)]
    pub struct Program {
        cfg: ProgramUiCfg,
    }
    
    impl Program {
        pub fn parse_serde_value(mut value: Value) -> Result<Self> {
            if !value.is_object() {
                return Err(Error::new(ErrorKind::JsonImportError, "json file has to have object type first".to_string()))
            }
            
    
            let cfg = match value["cfg"].take() {
                Value::String(s) => s,
                _ => return Err(Error::new(ErrorKind::JsonImportError, "".to_string())),
            };
    
            let pcfg = ProgramUiCfg::from_serde_value(&cfg, value);
    
            return match pcfg {
                Ok(o) => Ok(Program { cfg: o }),
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
    use super::error::*;
    use super::command_generator::KeybindSwitcher;

    pub fn import_json_cfg<P: AsRef<Path>>(path: P) -> Result<Value> {
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
