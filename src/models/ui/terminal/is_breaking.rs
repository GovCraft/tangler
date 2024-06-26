use std::fmt;
use std::ops::Deref;

use owo_colors::OwoColorize;
use serde::Deserialize;
use tracing::{error, instrument, trace};

use crate::models::{
    ALERT_COLOR,
    SCOPE_PUNCTUATION_COLOR,
};

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub(crate) struct IsBreakingTerminal(bool);


impl Deref for IsBreakingTerminal {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for IsBreakingTerminal {
    #[instrument(level = "trace", skip(self, f))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 {
            if let Err(e) = write!(
                f,
                "{}{}",
                "!".style(*ALERT_COLOR).bold(),
                ":".style(*SCOPE_PUNCTUATION_COLOR)
            ) {
                error!("{:?}", e);
            }
        } else if let Err(e) = write!(f, "{}", ":".style(*SCOPE_PUNCTUATION_COLOR)) {
            error!("{:?}", e);
        }


        Ok(())
    }
}

impl From<&bool> for IsBreakingTerminal {
    #[instrument(level = "info", skip(s))]
    fn from(s: &bool) -> Self {
        // Event: IsBreakingTerminal Created
        // IsBreakingTerminal: Triggered when a new IsBreakingTerminal instance is created from a bool.
        // Context: The string being converted to IsBreakingTerminal.
        trace!(source = %s, "IsBreakingTerminal instance created from bool");
        IsBreakingTerminal(*s)
    }
}
