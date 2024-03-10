mod manager;
pub use manager::*;

use std::path::PathBuf;

use serde::{
    Serialize,
    Deserialize,
};

use crate::lasso::{Preset, Rule, Matcher, AffinityMask};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LassoConfig {
    pub presets: Vec<Preset>,
    pub rules: Vec<Rule>,
    pub default_preset: Option<String>,
}

impl Default for LassoConfig {
    fn default () -> LassoConfig {
        let presets: Vec<Preset> = vec![
            Preset {
                name: Some(String::from("Cache")),
                description: Some(String::from("Uses cache cores 0-15 (best for gaming)")),
                affinity_mask: Some(AffinityMask(0x0000FFFF)),
                ..Preset::default()
            },
            Preset {
                name: Some(String::from("Performance")),
                description: Some(String::from("Uses performance cores 16-31 (best for productivity)")),
                affinity_mask: Some(AffinityMask(0xFFFF0000)),
                ..Preset::default()
            },
            Preset {
                name: Some(String::from("All")),
                description: Some(String::from("Uses all cores 0-31")),
                affinity_mask: Some(AffinityMask(0xFFFFFFFF)),
                ..Preset::default()
            },
        ];
        let rules: Vec<Rule> = vec![
            Rule {
                on: Matcher::Path(PathBuf::from("C:\\Program Files\\Steam\\steamapps\\common")),
                preset: String::from("Cache"),
                description: Some(String::from("Matches all processes in the Steam common folder"))
            },
            Rule {
                on: Matcher::Path(PathBuf::from("D:\\SteamLibrary\\steamapps\\common")),
                preset: String::from("Cache"),
                description: Some(String::from("Matches all processes in an external Steam library"))
            },
            Rule {
                on: Matcher::Path(PathBuf::from("D:\\Games")),
                preset: String::from("Cache"),
                description: Some(String::from("Matches all processes in your games folder"))
            },
        ];
        LassoConfig {
            default_preset: Some(String::from("performance")),
            presets,
            rules,
        }
    }
}