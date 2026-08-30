use serde::{Serialize, Deserialize};
use std::fs::{create_dir_all, write, read_to_string};
use std::io::{Error, Result};
use dirs::config_dir;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Setting {
    pub path: PathBuf,
    pub theme: bool,
}

impl Default for Setting {
    fn default() -> Self {
        let address = config_dir()
            .unwrap()
            .join("rqlab")
            .join("config.json");

        Self { 
            path: address, 
            theme: false 
        }
    }
}

pub fn update_setting(setting: &Setting) -> Result<()> {
    let tmp = serde_json::to_string(setting)
        .map_err(|error| Error::other(error))?;
    
    create_dir_all(setting.path.parent().unwrap())?;
    write(&setting.path, tmp)?;
    Ok(())
}

pub fn load_setting(setting: &Setting) -> Result<Setting> {
    let data = match read_to_string(&setting.path) {
        Ok(value) => value,
        Err(error) => return Err(error),
    };
    let config: Setting = serde_json::from_str(&data)
        .map_err(|error| Error::other(error))?;
    Ok(config)
}
