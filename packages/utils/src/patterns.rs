use regex::Regex;
use std::sync::LazyLock;
pub static ALPHA_DASH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._-]+$").unwrap());
