use std::fmt;

use serde::Deserialize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize)]
pub enum Severity {
    Unknown,
    Low,
    Medium,
    High,
    Critical,
}

impl Default for Severity {
    fn default() -> Self {
        Self::Unknown
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match *self {
            Self::Low => "Low risk",
            Self::Medium => "Medium risk",
            Self::High => "High risk",
            Self::Critical => "Critical risk",
            Self::Unknown => "Unknown risk",
        })
    }
}

impl Severity {
    pub const fn to_color(self) -> term::color::Color {
        match self {
            Self::Low => term::color::YELLOW,
            Self::Medium => term::color::BRIGHT_YELLOW,
            Self::High => term::color::RED,
            Self::Critical => term::color::BRIGHT_RED,
            _ => term::color::WHITE,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Deserialize)]
pub enum Status {
    Unknown,
    #[serde(rename = "Not affected")]
    NotAffected,
    Vulnerable,
    Fixed,
    Testing,
}

impl Default for Status {
    fn default() -> Self {
        Self::Unknown
    }
}
