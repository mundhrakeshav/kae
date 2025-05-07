use anyhow::{anyhow, Ok, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub const CONFIG_DIR_PATH: &str = ".todo";
pub const CONFIG_PATH: &str = ".todo/todo.json";

pub fn init_todos() -> Result<()> {
    if Path::new(CONFIG_PATH).exists() {
        return Err(anyhow!("Already initialized"));
    }

    if !Path::new(CONFIG_DIR_PATH).exists() {
        fs::create_dir(CONFIG_DIR_PATH)?;
    }

    let mut f = File::options()
        .read(true)
        .write(true)
        .create_new(true)
        .open(CONFIG_PATH)?;

    if let Err(e) = f.write(b"[]") {
        tracing::error!("error while writing to init file, err: {}", e);
        return Err(anyhow!("error while writing to init file, err: {}", e));
    };

    f.flush()?;

    Ok(())
}

pub fn read_todos_from(path: &str) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_todos_to(path: &str, tasks_json: String) -> Result<()> {
    let mut f = File::options()
        .write(true)
        .truncate(true) // Overwrite the file if it exists
        .create(true) // Create if it doesn't exist
        .open(path)?;

    if let Err(e) = f.write_all(tasks_json.as_bytes()) {
        tracing::error!("error while writing to file, err: {}", e);
        return Err(anyhow!("error while writing to file, err: {}", e));
    };

    f.flush()?;

    Ok(())
}
