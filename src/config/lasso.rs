use std::collections::HashMap;

use serde::{
    Serialize,
    Deserialize,
};

use super::AffinityMask;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LassoPreset {
    pub name: Option<String>,
    pub description: Option<String>,
    pub affinity_mask: Option<AffinityMask>,
    pub priority: Option<usize>,
}

impl Default for LassoPreset {
    fn default () -> LassoPreset {
        LassoPreset {
            name: None,
            description: None,
            affinity_mask: None,
            priority: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum LassoMatcher {
    Path(String),
    Name(String),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LassoRule {
    pub on: LassoMatcher,
    pub preset: String,
    pub description: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LassoConfig {
    pub default_preset: Option<String>,
    pub presets: HashMap<String, LassoPreset>,
    pub rules: Vec<LassoRule>,
}

impl Default for LassoConfig {
    fn default () -> LassoConfig {
        let rules: Vec<LassoRule> = vec![
            LassoRule {
                on: LassoMatcher::Path(String::from("C:\\Program Files\\Steam\\steamapps\\common")),
                preset: String::from("cache"),
                description: Some(String::from("Matches all processes in the Steam common folder"))
            },
            LassoRule {
                on: LassoMatcher::Path(String::from("D:\\SteamLibrary\\steamapps\\common")),
                preset: String::from("cache"),
                description: Some(String::from("Matches all processes in an external Steam library"))
            },
            LassoRule {
                on: LassoMatcher::Path(String::from("D:\\Games")),
                preset: String::from("cache"),
                description: Some(String::from("Matches all processes in your games folder"))
            },
        ];
        let mut presets: HashMap<String, LassoPreset> = HashMap::new();
        presets.insert(String::from("cache"), LassoPreset {
            name: Some(String::from("Cache")),
            description: Some(String::from("Uses cache cores 0-15 (best for gaming)")),
            affinity_mask: Some(AffinityMask(0x0000FFFF)),
            ..LassoPreset::default()
        });
        presets.insert(String::from("performance"), LassoPreset {
            name: Some(String::from("Performance")),
            description: Some(String::from("Uses performance cores 16-31 (best for productivity)")),
            affinity_mask: Some(AffinityMask(0xFFFF0000)),
            ..LassoPreset::default()
        });
        presets.insert(String::from("all"), LassoPreset {
            name: Some(String::from("All")),
            description: Some(String::from("Uses all cores 0-31")),
            affinity_mask: Some(AffinityMask(0xFFFFFFFF)),
            ..LassoPreset::default()
        });
        LassoConfig {
            default_preset: Some(String::from("performance")),
            presets,
            rules,
        }
    }
}