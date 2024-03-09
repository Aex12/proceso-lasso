use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PLConfigProfile {
    pub name: String,
    pub description: Option<String>,
    pub affinity_mask: Option<u64>,
    pub priority: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PLConfigRule {
    pattern: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PLConfig {
    pub default_profile: Option<String>,
    pub profiles: Vec<PLConfigProfile>,
    pub rules: Vec<PLConfigRule>,
}