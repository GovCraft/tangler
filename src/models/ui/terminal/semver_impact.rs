use std::fmt;

use owo_colors::OwoColorize;
use tracing::{error, instrument, trace};

use crate::models::*;
use crate::models::semver_impact::SemVerImpact;

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct SemVerImpactTerminal(SemVerImpact);


impl fmt::Display for SemVerImpactTerminal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self.0.to_string().as_str() {
            "MAJOR" => write!(f, "{}", "MAJOR".style(*MAJOR)),
            "MINOR" => write!(f, "{}", "MINOR".style(*MINOR)),
            "PATCH" => write!(f, "{}", "PATCH".style(*PATCH)),
            _ => write!(f, "{}", "\u{2022}".style(*PUNCTUATION_COLOR)),
        };

        if let Err(e) = result {
            error!("{:?}", e);
        }

        Ok(())
    }
}

impl From<&SemVerImpact> for SemVerImpactTerminal {
    #[instrument(level = "info", skip(s))]
    fn from(s: &SemVerImpact) -> Self {
        // Event: SemVerImpactTerminal Created
        // SemVerImpactTerminal: Triggered when a new SemVerImpactTerminal instance is created from a &str.
        // Context: The string being converted to SemVerImpactTerminal.
        trace!(source = %s, "SemVerImpactTerminal instance created from &str");
        SemVerImpactTerminal(s.clone())
    }
}
