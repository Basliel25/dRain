use serde::{Serialize, Deserialize};
use crate::template::{Template, TemplateId};

/// On-disk snapshot of the drain learned state
#[derive(Serialize, Deserialize, Debug)]
pub struct DrainSnapshot {
    /// Version of the snapshot
    pub version: u64,
    /// Similarity Threshold the tree was
    /// traind in
    pub threshold: f64,
    /// Next template id the snapshot will capture
    pub next_id: TemplateId,

    /// Vector of all minted templates
    pub templates: Vec<Template>,
}

impl DrainSnapshot {
    pub const CURRENT_VERSION: u32 = 1;
}
