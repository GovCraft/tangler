use std::path::PathBuf;
use akton::prelude::*;
use derive_more::*;
use derive_new::new;
use crate::models::{CommittedCommit, Oid, TimeStamp};

/// Represents a successful commit message with its details.
#[derive(new, Default, Debug, Clone, )]
pub(crate) struct FileChangeDetected {
    pub(crate) path: PathBuf,
}


