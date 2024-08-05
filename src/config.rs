use config::{Config, ConfigError, File, FileFormat};
use directories::UserDirs;
use std::collections::HashMap;
use std::path::PathBuf;

// pub struct Config {
//     // pub api_key: String,
//     // pub api_base: String,
//     // pub shell: String,
// }
// impl Config {
//     pub fn new() -> Self {
//         Self {}
//     }
// }

fn get_ahh_dir() -> PathBuf {
    let user_dirs = UserDirs::new().expect("Failed to get user directories");
    let config_dir = user_dirs.home_dir().join(".ahh");
    println!("{:?}", config_dir);
    return config_dir;
}

fn load_config_internal() -> Result<Config, ConfigError> {
    let config_dir = get_ahh_dir();
    let config_path: PathBuf = config_dir.join(".ahh.toml");
    // Check if the file exists
    if !config_path.exists() {
        return Err(ConfigError::NotFound(config_path.to_string_lossy().into()));
    }

    let toml_config = config_path
        .to_str()
        .ok_or_else(|| ConfigError::Message("Invalid path".to_string()))?;

    let config = Config::builder()
        .add_source(File::new(toml_config, FileFormat::Toml))
        .build();
    config
}

pub(crate) fn load_config() -> HashMap<String, String> {
    let config_result = load_config_internal();

    match config_result {
        Ok(config) => {
            // Convert Config to HashMap
            config
                .try_deserialize::<HashMap<String, String>>()
                .unwrap_or_default()
        }
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            // Return empty HashMap on error
            HashMap::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::load_config;

    #[test]
    fn test_check_config() {
        let config = load_config();
        println!("{:?}", config);
        let model = config.get("model").unwrap();
        println!("{}", model)
    }
}
