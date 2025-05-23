use anyhow::Result;
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use std::env;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
use tracing::info;
use validator::{ValidationError, ValidationErrors};
pub mod constants;
pub mod environment;
pub mod patterns;

pub fn codify(value: &str) -> String {
    value.to_owned().replace(" ", "_").to_ascii_uppercase()
}

pub fn dt_human(timestamp: DateTime<Utc>, timezone: &Tz) -> String {
    let dt_local = timestamp.with_timezone(timezone);
    dt_local.format("%B %d, %Y at %I:%M %p").to_string() // Ad
}

pub fn verrors(field: &'static str, code: &'static str, message: String) -> ValidationErrors {
    ValidationErrors::new().with_error(
        field,
        ValidationError::new(code).with_message(message.into()),
    )
}

pub fn touch(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        OpenOptions::new().create(true).write(true).open(path)?;
    }
    Ok(())
}

pub fn load_dotenvs(dotenvs: Vec<PathBuf>) -> Result<()> {
    for dotenv in dotenvs {
        if dotenv.exists() {
            // loads the .env, then overrides with current environment so .env can be ovwridden
            info!("Loading environment file: {}", &dotenv.to_string_lossy());
            dotenvy::from_path(&dotenv)?;
        }
    }
    Ok(())
}

pub fn resolve_path<P: AsRef<Path>>(path: &P) -> PathBuf {
    let path = path.as_ref();

    if path.is_absolute() {
        path.to_path_buf()
    } else {
        let base_path = cwd();
        // cargo tauri dev changes directories
        base_path.join(path)
    }
}

// should only be used for dev, prod paths should be known or configured
// only really usable by tauri and maybe dioxus
// TODO: better solution
pub fn cwd() -> PathBuf {
    let mut current = env::current_dir().unwrap();
    // we're in the wrong directory
    if current.join("tauri.conf.json").exists() {
        current.pop(); // move to packages
        current.pop(); // move to workspace root
    }
    current
}

pub fn find_workspace_root() -> Option<PathBuf> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").ok()?;
    let mut path = PathBuf::from(manifest_dir);

    loop {
        if !path.pop() {
            return None;
        }

        let cargo_toml = path.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return Some(path);
                }
            }
        }
    }
}
