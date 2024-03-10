use serde::{Deserialize, Serialize};
pub use super::AffinityMask;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Preset {
    pub name: Option<String>,
    pub description: Option<String>,
    pub affinity_mask: Option<AffinityMask>,
    pub priority: Option<usize>,
}

impl Default for Preset {
    fn default () -> Preset {
        Preset {
            name: None,
            description: None,
            affinity_mask: None,
            priority: None,
        }
    }
}