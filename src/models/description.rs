use std::fmt;
use std::ops::Deref;

use serde::Deserialize;
use tracing::{instrument, trace};

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub(crate) struct Description(String);


impl Deref for Description {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Description {
    #[instrument(level = "trace", skip(self, f))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let description = self.0.to_string();
        let result = write!(f, "{}", description);

        result
    }
}

impl From<&str> for Description {
    #[instrument(level = "info", skip(s))]
    fn from(s: &str) -> Self {
        // Event: Description Created
        // Description: Triggered when a new Description instance is created from a &str.
        // Context: The string being converted to Description.
        trace!(source = %s, "Description instance created from &str");
        Description(s.to_string())
    }
}
