use std::path::Path;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use serde_json::Value;
use crate::error::*;
use crate::keybind_switcher::KeybindSwitcher;

pub fn import_json<P: AsRef<Path>>(path: P) -> Result<Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let val = serde_json::from_reader::<BufReader<File>, Value>(reader)?;
    Ok(val)
}

pub fn export_switcher<P: AsRef<Path>>(path: P, kbsw: KeybindSwitcher) -> Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    let writer = BufWriter::new(file);

    match serde_json::to_writer(writer, &kbsw) {
        Ok(o) => Ok(o),
        Err(e) => Err(e.into()),
    }
}

pub fn import_switche<P: AsRef<Path>>(path: P) -> Result<KeybindSwitcher> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let kbsw = serde_json::from_reader(reader)?;
    Ok(kbsw)
}
