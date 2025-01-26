use anyhow::Result;
use std::env;
use std::path::{Path, PathBuf};
use tracing::info;

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
pub fn cwd() -> PathBuf {
    let mut current = env::current_dir().unwrap();
    // we're in the wrong directory
    if current.join("tauri.conf.json").exists() {
        current.pop(); // returns false if root
    }
    current
}
