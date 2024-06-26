use std::fmt;
use std::ops::Deref;

use serde::Deserialize;
use tracing::*;

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
pub(crate) struct Oid(String);


impl Oid {
    pub(crate) fn new(oid: &str) -> Oid {
        Oid(oid.to_string())
    }
}

impl Deref for Oid {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Oid {
    #[instrument(level = "trace", skip(self, f))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let short_oid = &self.0[..7].to_lowercase();
        write!(f, "{short_oid}")
    }
}

impl From<&str> for Oid {
    #[instrument(level = "info", skip(s))]
    fn from(s: &str) -> Self {
        trace!(source = %s, "Oid instance created from &str");
        Oid(s.to_string())
    }
}
