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

pub async fn manage<R: Runtime>(app: &mut App<R>, config: Option<AppConfig>) -> Result<(), Error> {
    let conf: AppConfig;
    if config.is_none() {
        conf = AppConfig::from_app(app)?;
    } else {
        conf = config.unwrap();
    }
    app.manage(conf);
    Ok(())
}

// Configuration structure
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub app_env: Environment,
    pub data_dir: PathBuf,
    pub db_path: PathBuf,
    pub logs_dir: PathBuf,
    pub no_dotenv: bool,
    pub no_migrate: bool,
    pub no_fs_logging: bool,
    pub should_seed_dev: bool,
    pub no_logging_filer_reload: bool,
}

impl AppConfig {
    pub fn from_app<R: Runtime>(app: &App<R>) -> Result<Self, Error> {
        let mut config = AppConfig::default_from_app(app);
        let matches = AppConfig::matches(app)?;
        config.load(matches)?; // apply matches or load from env
        config.no_dotenv = Self::should_no_dotenv(app)?;
        Ok(config)
    }

    pub fn default_from_app<R: Runtime>(app: &App<R>) -> Self {
        // should be absolute
        let data_dir = app
            .path()
            .app_data_dir()
            .expect("Unable to determine app data dir");
        AppConfig::default(&data_dir)
    }

    pub fn default(data_dir: &PathBuf) -> Self {
        AppConfig {
            data_dir: data_dir.clone(),
            app_env: Environment::default(),
            db_path: data_dir.clone().join("app.db"),
            logs_dir: data_dir.clone().join("logs"),
            ..Default::default()
        }
    }

    pub fn should_no_dotenv<R: Runtime>(app: &App<R>) -> Result<bool> {
        let matches = Self::matches(app)?;
        if let Some(skipval) = AppConfig::get_arg::<bool>(&matches, "skip-dotenv", "NO_DOTENV") {
            return Ok(skipval);
        }
        return Ok(false);
    }

    pub fn load(&mut self, matches: HashMap<String, Value>) -> Result<()> {
        if let Some(val) = Self::get_arg::<String>(&matches, "logs", "LOGS_DIR") {
            self.logs_dir = resolve_path(&val);
        }
        if let Some(val) = Self::get_arg::<String>(&matches, "db", "DATABASE_PATH") {
            self.db_path = resolve_path(&val);
        }
        if let Some(val) = Self::get_arg::<String>(&matches, "env", "APP_ENV") {
            self.app_env = Environment::from_str(&val)?;
        }
        if let Some(val) = Self::get_arg::<bool>(&matches, "_", "NO_MIGRATE") {
            self.no_migrate = val
        }
        if let Some(val) = Self::get_arg::<bool>(&matches, "_", "NO_FS_LOGGING") {
            self.no_fs_logging = val
        }
        Ok(())
    }

    // Exclude tests from CLI as it messes it'll take in cargo options
    #[cfg(any(target_os = "ios", target_os = "android"))]
    pub fn matches<R: Runtime>(_app: &App<R>) -> Result<HashMap<String, Value>> {
        Ok(HashMap::new())
    }

    // Matches can fail. It relies on the cli plugin existing and is sort of strict on what it allows
    #[allow(unused_variables)]
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    pub fn matches<R: Runtime>(app: &App<R>) -> Result<HashMap<String, Value>> {
        match app.cli().matches() {
            Ok(matches) => Ok(matches.args.iter().map(|(key, arg_data)| (key.clone(), arg_data.value.clone()))
            .collect()),
            Err(e) => {
                #[cfg(test)]
                {
                    Ok(HashMap::new())
                }
                #[cfg(not(test))]
                {
                    Err(Error::from(e))

                }
            }
        }

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
