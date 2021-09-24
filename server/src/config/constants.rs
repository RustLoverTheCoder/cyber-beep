use once_cell::sync::Lazy;
use regex::Regex;

pub static USERNAME_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^[a-zA-Z0-9_-]{4,16}$").unwrap());

pub static PASSWORD_RE: Lazy<Regex> = Lazy::new(|| Regex::new("^[a-zA-Z0-9_-]{6,16}$").unwrap());
