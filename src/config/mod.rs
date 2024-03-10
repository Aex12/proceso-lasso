mod manager;
pub use manager::*;

use std::{collections::HashMap, path::PathBuf};

use serde::{
    Serialize,
    Deserialize,
};

use crate::lasso::{Preset, Rule, Matcher, AffinityMask};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub presets: HashMap<String, Preset>,
    pub rules: Vec<Rule>,
    pub default_preset: String,
}

impl Config {
    pub fn get_preset (&self, name: &str) -> Option<&Preset> {
        self.presets.get(name)
    }

    pub fn find_rule (&self, process: &crate::process::Process) -> Option<&Rule> {
        self.rules.iter().find(|rule| rule.on.matches(process))
    }

    pub fn validate (&self) -> Result<(), String> {
        for rule in &self.rules {
            if !self.presets.contains_key(&rule.preset) {
                return Err(format!("Rule {:?} references non-existent preset {}", rule, rule.preset));
            }
        }
        if !self.presets.contains_key(&self.default_preset) {
            return Err(format!("Default preset {} does not exist", &self.default_preset));
        }
        Ok(())
    }
}

impl Default for Config {
    fn default () -> Config {
        let mut presets: HashMap<String, Preset> = HashMap::new();
        presets.insert(
            String::from("Cache"),
            Preset {
                description: Some(String::from("Uses cache cores 0-15 (best for gaming)")),
                affinity: Some(AffinityMask(0x0000FFFF)),
                ..Preset::default()
            },
        );
        presets.insert(
            String::from("Performance"),
            Preset {
                description: Some(String::from("Uses performance cores 16-31 (best for productivity)")),
                affinity: Some(AffinityMask(0xFFFF0000)),
                ..Preset::default()
            },
        );
        presets.insert(
            String::from("All"),
            Preset {
                description: Some(String::from("Uses all cores 0-31")),
                affinity: Some(AffinityMask(0xFFFFFFFF)),
                ..Preset::default()
            },
        );
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
        Config {
            default_preset: String::from("Performance"),
            presets,
            rules,
        }
    }
}