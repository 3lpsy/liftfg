use anyhow::{Error, Result};

use fgcore::environment::Environment;
use fgcore::utils::resolve_path;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::str::FromStr;
use std::{collections::HashMap, path::PathBuf};
use tauri::{App, Manager, Runtime};

#[cfg(not(any(target_os = "ios", target_os = "android")))]
use tauri_plugin_cli::CliExt;

pub async fn setup<R: Runtime>(app: &mut App<R>) -> Result<(), Error> {
    let config = AppConfig::new(app)?;
    app.manage(config);
    Ok(())
}

// Configuration structure
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub app_env: Environment,
    pub db_path: PathBuf,
    pub logs_dir: PathBuf,
    pub skip_dotenv: bool,
}

impl AppConfig {
    pub fn new<R: Runtime>(app: &App<R>) -> Result<Self, Error> {
        let mut config = AppConfig::default(app);
        let matches = AppConfig::matches(app);
        config.load(matches)?; // apply matches or load from env
        Ok(config)
    }

    pub fn default<R: Runtime>(app: &App<R>) -> Self {
        // should be absolute
        let data_dir = app.path().app_data_dir().unwrap();
        AppConfig {
            app_env: Environment::default(),
            db_path: data_dir.join("app.db"),
            logs_dir: data_dir.join("logs"),
            skip_dotenv: false,
        }
    }

    pub fn should_skip_dotenv<R: Runtime>(app: &App<R>) -> bool {
        let matches = Self::matches(app);
        if let Some(skipval) = AppConfig::get_arg::<bool>(&matches, "skip-dotenv", "SKIP_DOTENV") {
            return skipval;
        }
        return false;
    }

    pub fn load(&mut self, matches: HashMap<String, Value>) -> Result<()> {
        if let Some(val) = Self::get_arg::<String>(&matches, "logs", "LOGS_PATH") {
            self.logs_dir = resolve_path(&val);
        }
        if let Some(val) = Self::get_arg::<String>(&matches, "db", "DATABASE_PATH") {
            self.db_path = resolve_path(&val);
        }
        if let Some(val) = Self::get_arg::<String>(&matches, "env", "APP_ENV") {
            self.app_env = Environment::from_str(&val)?;
        }
        Ok(())
    }

    #[cfg(any(target_os = "ios", target_os = "android"))]
    pub fn matches<R: Runtime>(_app: &App<R>) -> HashMap<String, Value> {
        HashMap::new()
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub fn matches<R: Runtime>(app: &App<R>) -> HashMap<String, Value> {
        app.cli()
            .matches()
            .unwrap()
            .args
            .iter()
            .map(|(key, arg_data)| (key.clone(), arg_data.value.clone()))
            .collect()
    }

    pub fn get_arg<T: FromValue>(
        matches: &HashMap<String, Value>,
        key: &str,
        env_key: &str,
    ) -> Option<T> {
        if let Some(arg) = matches.get(key) {
            match arg {
                // if it's false bool from the cli, we fallback to env explicitly
                Value::Bool(false) => {}
                _ => {
                    if let Some(value) = T::from_value(arg) {
                        return Some(value);
                    }
                }
            }
        }
        if let Ok(val) = env::var(env_key) {
            if !val.is_empty() {
                return T::from_string(&val);
            }
        }
        None
    }
}

pub trait FromValue: Sized {
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
