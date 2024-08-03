use config::{Config, File, FileFormat};
use directories::UserDirs;
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

fn load_config() -> Result<Config, config::ConfigError> {
    let config_dir = get_ahh_dir();
    let config_path: PathBuf = config_dir.join(".ahh.toml");

    let config = Config::builder()
        .add_source(File::new(config_path.to_str().unwrap(), FileFormat::Toml))
        .build();
    config
}

#[cfg(test)]
mod tests {
    use crate::config::load_config;
    use std::collections::HashMap;

    #[test]
    fn test_check_config() {
        let settings = load_config().unwrap();
        let config = settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap();
        println!("{:?}", config)
        // match load_config() {
        //     Ok(config) => {
        //         let model = config.get_string("model").unwrap();
        //         println!("model {}", model)
        //     }
        //     Err(e) => eprintln!("Error loading config: {}", e),
        // }
    }
}
