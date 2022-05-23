use crate::{error::*, GenerateCommand};
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod test {
    use crate::{keybind_switcher::{KeybindSwitcher, CommandSet}, GenerateCommand};

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
