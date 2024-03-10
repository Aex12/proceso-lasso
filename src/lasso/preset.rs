use serde::{Deserialize, Serialize};
pub use super::AffinityMask;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub description: Option<String>,
    pub affinity_mask: Option<AffinityMask>,
    pub priority: Option<usize>,
}

impl Default for Preset {
    fn default () -> Preset {
        Preset {
            description: None,
            affinity_mask: None,
            priority: None,
        }
    }
}