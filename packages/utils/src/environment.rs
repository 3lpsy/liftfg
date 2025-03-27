use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Platform {
    IOS,
    Android,
    Linux,
    Mac,
    Windows,
}

impl Platform {
    pub fn from_ua(ua: &str) -> Self {
        let ua = ua.to_lowercase();
        if ua.contains("android") {
            Platform::Android
        } else if ua.contains("iphone") || ua.contains("ipad") || ua.contains("ipod") {
            Platform::IOS
        } else if ua.contains("macintosh") || ua.contains("mac os") {
            Platform::Mac
        } else if ua.contains("linux") {
            Platform::Linux
        } else if ua.contains("windows") {
            Platform::Windows
        } else {
            panic!("Unknown platform: {}", ua);
        }
    }
    pub fn is_mobile(&self) -> bool {
        matches!(self, Platform::IOS | Platform::Android)
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, Platform::Linux | Platform::Mac | Platform::Windows)
    }
}
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Environment {
    Dev,
    Prod,
    Testing,
}

impl FromStr for Environment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dev" => Ok(Environment::Dev),
            "prod" => Ok(Environment::Prod),
            "testing" | "test" => Ok(Environment::Testing),
            _ => Err(anyhow!(
                "Invalid environment: '{}'. Use either 'dev' or 'prod'.",
                s
            )),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Dev
    }
}
