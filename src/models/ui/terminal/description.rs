use std::fmt;
use std::ops::Deref;

use owo_colors::OwoColorize;
use serde::Deserialize;
use tracing::{error, instrument, trace};

use crate::models::{
    Description, DESCRIPTION_COLOR,
};

#[derive(Debug, Default, Clone, Deserialize, PartialEq)]
pub(crate) struct DescriptionTerminal(Description);


impl Deref for DescriptionTerminal {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for DescriptionTerminal {
    #[instrument(level = "trace", skip(self, f))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Err(e) = write!(f, "{}", self.0.style(*DESCRIPTION_COLOR)) {
            error!("{:?}", e);
        }
        Ok(())
    }
}

impl From<&Description> for DescriptionTerminal {
    #[instrument(level = "info", skip(s))]
    fn from(s: &Description) -> Self {
        // Event: DescriptionTerminal Created
        // DescriptionTerminal: Triggered when a new DescriptionTerminal instance is created from a &str.
        // Context: The string being converted to DescriptionTerminal.
        trace!(source = %s, "DescriptionTerminal instance created from &str");
        DescriptionTerminal(s.clone())
    }
}
