use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
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
