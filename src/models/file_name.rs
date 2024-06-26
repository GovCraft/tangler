use std::fmt;
use std::ops::Deref;

use serde::Deserialize;
use tracing::{instrument, trace};

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Filename(String);


impl Deref for Filename {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Filename {
    #[instrument(level = "trace", skip(self, f))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(last_segment) = self.0.rsplit('/').next() {
            write!(f, "{}", last_segment)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl From<&str> for Filename {
    #[instrument(level = "info", skip(s))]
    fn from(s: &str) -> Self {
        // Event: Filename Created
        // Filename: Triggered when a new Filename instance is created from a &str.
        // Context: The string being converted to Filename.
        trace!(source = %s, "Filename instance created from &str");
        Filename(s.to_string())
    }
}
