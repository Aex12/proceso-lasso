use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PLConfigProfile {
    pub name: String,
    pub description: Option<String>,
    pub affinity_mask: Option<u64>,
    pub priority: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PLConfigRule {
    PathRule {
        path: String,
        profile: String,
    },
    NameRule {
        name: String,
        profile: String,
    },
    PatternRule {
        pattern: String,
        profile: String,
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PLConfig {
    pub default_profile: Option<String>,
    pub profiles: HashMap<String, PLConfigProfile>,
    pub rules: Vec<PLConfigRule>,
}

impl PLConfig {
    pub fn default () -> PLConfig {
        PLConfig {
            default_profile: None,
            profiles: HashMap::new(),
            rules: Vec::new(),
        }
    }
}