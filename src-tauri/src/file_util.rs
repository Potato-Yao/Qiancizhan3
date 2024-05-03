use std::{env, fs, io};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[tauri::command]
pub fn get_wordbooks() -> HashMap<String, String> {
    let dir_path = env::current_exe().unwrap().parent().unwrap().to_path_buf().join("qiancizhan-resources");
    let entries = fs::read_dir(dir_path).unwrap();

    let mut db_files_map: HashMap<String, String> = HashMap::new();
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                if let Some(name_str) = file_name.to_str() {
                    if path.extension().map_or(false, |ext| ext == "db") {
                        db_files_map.insert(name_str.to_string(), path.to_str().unwrap().to_string());
                    }
                }
            }
        }
    }

    db_files_map
}

pub fn get_or_create_resource_dir() -> Result<PathBuf, io::Error> {
    let exe_path = env::current_exe()?;
    let mut resource_dir = exe_path.parent().unwrap().to_path_buf();
    resource_dir.push("qiancizhan-resources");

    if !resource_dir.exists() {
        fs::create_dir(&resource_dir)?;
        let database_path = env::current_dir()?.join("resources").join("test1.db");
        let des_path = resource_dir.clone().join("test1.db");
        fs::copy(&database_path, &des_path)?;
        println!("{:?}, {:?}", database_path, des_path);
    }

    #[cfg(test)]
    {}

    let mut config_path = resource_dir.clone();
    config_path.push("config.toml");
    if !config_path.exists() {
        let default_config = include_str!("../resources/default_config.toml");
        let mut file = File::create(config_path)?;
        file.write_all(default_config.as_ref())?;
    }


    Ok(resource_dir)
}

#[cfg(test)]
mod tests {
    use crate::file_util::get_or_create_resource_dir;

    #[test]
    fn test_custom_resource_dir() {
        let resource_dir = get_or_create_resource_dir().unwrap();
        println!("{:?}", resource_dir);
    }
}
