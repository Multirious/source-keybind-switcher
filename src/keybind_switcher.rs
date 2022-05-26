use crate::{error::*, GenerateCommand};
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod test {
    use crate::{keybind_switcher::{KeybindSwitcher, CommandSet, CommandBind}, GenerateCommand};

    #[test]
    fn test() {
        fn s(str: &str) -> String {
            str.to_string()
        }

        let cmds = vec![
            CommandSet::new(s("first"), s("third"), s("second"), vec![CommandBind::new(s("a"), s("echo kill")), CommandBind::new(s("s"), s("echo sex"))]),
            CommandSet::new(s("second"), s("first"), s("third"), vec![CommandBind::new(s("a"), s("echo blowjob")), CommandBind::new(s("a"), s("echo givehead"))]),
            CommandSet::new(s("third"), s("second"), s("first"), vec![CommandBind::new(s("a"), s("echo drink)")), CommandBind::new(s("a"), s("echo eat"))]),
        ];

        let kbsw = KeybindSwitcher::new(s("test"), s("pgup"), s("pgdn"), cmds);

        let g = kbsw.generate().unwrap();

        println!("{}", g);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeybindSwitcher {
    pub name: String,
    pub key_next: String,
    pub key_previous: String,
    pub command_sets: Vec<CommandSet>
}

impl KeybindSwitcher {
    pub fn new(name: String, key_next: String, key_previous: String, command_sets: Vec<CommandSet>) -> Self {
        Self { name, key_next, key_previous, command_sets }
    }

    pub fn index_command_sets_by_index(&mut self) {
        let len = self.command_sets.len() as i32;
        for i in 0..len {
            let prev_idx = (i - 1).rem_euclid(len);
            let next_idx = (i + 1).rem_euclid(len);

            let prev_set = self.command_sets[prev_idx as usize].name.clone();
            let next_set = self.command_sets[next_idx as usize].name.clone();

            let c = &mut self.command_sets[i as usize];
            c.previous_set = prev_set;
            c.next_set = next_set;
        }            
    }
}

impl Default for KeybindSwitcher {
    fn default() -> Self {
        Self { name: Default::default(), key_next: Default::default(), key_previous: Default::default(), command_sets: Default::default() }
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
            
            for (i, c) in cs.command_binds.iter().enumerate() {
                let name = format!("{}_cmd{}", alias_curr, i);
                add_line(&mut s, &format!(r#"alias "{}" {}"#, name, c.generate()?));
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandBind {
    pub key: String,
    pub command: String,
}

impl CommandBind {
    pub fn new(key: String, command: String) -> Self {
        Self { key, command }
    }
}

impl Default for CommandBind {
    fn default() -> Self {
        Self { key: Default::default(), command: Default::default() }
    }
}

impl GenerateCommand for CommandBind {
    fn generate(&self) -> Result<String> {
        if self.key.is_empty() {
            return Err(Error::new(ErrorKind::FieldEmpty, "CommandBind's key field is empty".to_string()))
        }
        if self.command.is_empty() {
            return Err(Error::new(ErrorKind::FieldEmpty, "CommandBind's command field is empty".to_string()))
        }
        
        Ok(format!("bind {} {}", self.key, self.command))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandSet {
    pub name: String,
    pub previous_set: String,
    pub next_set: String,
    pub command_binds: Vec<CommandBind>,
}

impl CommandSet {
    pub fn new(name: String, previous_set: String, next_set: String, command_binds: Vec<CommandBind>) -> Self {
        Self { name, previous_set, next_set, command_binds }
    }
}

impl Default for CommandSet {
    fn default() -> Self {
        Self { name: Default::default(), previous_set: Default::default(), next_set: Default::default(), command_binds: Default::default() }
    }
}