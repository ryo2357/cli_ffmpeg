use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::APP_NAME;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_string_test", skip_serializing_if = "is_default_string")]
    string_test: String,
    #[serde(default = "default_output_dir", skip_serializing_if = "is_default_path")]
    dir_test: PathBuf,
}

fn default_string_test() -> String {
    "default_string".to_string()
}

fn is_default_string(value: &String) -> bool {
  *value == default_string_test()
}


fn default_output_dir() -> PathBuf {
    dirs::document_dir().unwrap_or_else(|| PathBuf::from("C:/test"))
}

fn is_default_path(value: &PathBuf) -> bool {
  *value == default_output_dir()
}

impl Config {
    pub fn load() -> Self {
      let config_dir = dirs::config_dir()
        .expect("Failed to get config directory")
        .join(APP_NAME);
      let config_file = config_dir.join("config.toml");

      if !config_dir.exists() {
          fs::create_dir_all(&config_dir).expect("Failed to create config directory");
      }

      let config_content = if config_file.exists() {
          fs::read_to_string(&config_file).expect("Failed to read config file")
      } else {
          let default_config = Config {
            string_test: default_string_test(),
            dir_test: default_output_dir(),
          };
          let toml_string = toml::to_string(&default_config).expect("Failed to serialize config");
          fs::write(&config_file, toml_string.clone()).expect("Failed to write config file");
          toml_string
      };

      toml::from_str(&config_content).expect("Failed to parse config")
    }
}
