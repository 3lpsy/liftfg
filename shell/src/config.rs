use anyhow::{Error, Result};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::str::FromStr;
use std::{collections::HashMap, path::PathBuf};
use tauri::{App, Manager};

#[cfg(not(any(target_os = "ios", target_os = "android")))]
use tauri_plugin_cli::CliExt;

// Configuration structure
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub db_path: PathBuf,
    pub logs_dir: PathBuf,
}

impl AppConfig {
    pub fn new(app: &App) -> Result<Self, Error> {
        let mut config = AppConfig::default(app);
        let matches = AppConfig::matches(app);
        config.load(matches);

        Ok(config)
    }
    pub fn default(app: &App) -> Self {
        let data_dir = app.path().app_data_dir().unwrap();
        AppConfig {
            db_path: data_dir.join("app.db"),
            logs_dir: data_dir.join("logs"),
        }
    }
    pub fn load(&mut self, matches: HashMap<String, Value>) {
        let logs: Option<String> = get_arg(&matches, "logs", "LOGS_PATH");
        if logs.is_some() {
            self.db_path = PathBuf::from_str(&logs.unwrap()).unwrap()
        }
        let db: Option<String> = get_arg(&matches, "db", "DATABASE_PATH");
        if db.is_some() {
            self.db_path = PathBuf::from_str(&db.unwrap()).unwrap()
        }
    }

    #[cfg(any(target_os = "ios", target_os = "android"))]
    fn matches(_app: &App) -> HashMap<String, Value> {
        HashMap::new()
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    fn matches(app: &App) -> HashMap<String, Value> {
        app.cli()
            .matches()
            .unwrap()
            .args
            .iter()
            .map(|(key, arg_data)| (key.clone(), arg_data.value.clone()))
            .collect()
    }
}

trait FromValue: Sized {
    fn from_value(value: &Value) -> Option<Self>;
    fn from_string(s: &str) -> Option<Self>;
}

// Implement FromValue for String
impl FromValue for String {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }
    fn from_string(s: &str) -> Option<Self> {
        Some(s.to_string())
    }
}

// Implement FromValue for bool
impl FromValue for bool {
    fn from_value(value: &Value) -> Option<Self> {
        match value {
            Value::Bool(b) => Some(*b),
            Value::String(s) => bool::from_str(s).ok(), // Also try parsing from string
            _ => None,
        }
    }
    fn from_string(s: &str) -> Option<Self> {
        bool::from_str(s).ok()
    }
}

fn get_arg<T: FromValue>(matches: &HashMap<String, Value>, key: &str, env_key: &str) -> Option<T> {
    let cli = matches.get(key).and_then(|arg| T::from_value(&arg));
    if cli.is_some() {
        return cli;
    }
    if let Ok(val) = env::var(env_key) {
        if !val.is_empty() {
            return T::from_string(&val);
        }
    }
    None
}

pub async fn setup(app: &mut App) -> Result<(), Error> {
    let config = AppConfig::new(app)?;
    app.manage(config);
    Ok(())
}
