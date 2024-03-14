use thiserror::Error;

use std::collections::HashSet;
use std::{collections::HashMap, path::PathBuf};

use serde::{
    Serialize,
    Deserialize,
};

use super::rule::RuleMatch;
use super::{AffinityMask, Matchable, Matcher, Preset, Rule};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub presets: HashMap<String, Preset>,
    pub rules: Vec<Rule>,
    pub default_preset: String,
}

#[derive(Error, Debug)]
pub enum ConfigValidationError {
    #[error("Default preset couldnt be found {0}")]
    NonExistentDefaultPreset(String),
    #[error("Rule points to non-existent preset {0}")]
    NonExistentPreset(Rule),
    #[error("Duplicate rule at index {0} {1} {2}")]
    DuplicateRule(usize, Rule, Rule),
}

impl Config {
    pub fn get_preset (&self, name: &str) -> Option<&Preset> {
        self.presets.get(name)
    }

    pub fn validate_default_preset (&self) -> Result<(), ConfigValidationError> {
        if !self.presets.contains_key(&self.default_preset) {
            return Err(ConfigValidationError::NonExistentDefaultPreset(self.default_preset.clone()));
        }
        Ok(())
    }
    
    pub fn validate_rules (&self) -> Result<(), ConfigValidationError> {
        let mut seen_rules: HashSet<&Rule> = HashSet::new();
        
        for (index, rule) in self.rules.iter().enumerate() {
            if !self.presets.contains_key(&rule.preset) {
                return Err(ConfigValidationError::NonExistentPreset(rule.clone()));
            }
            if seen_rules.contains(rule) {
                let prev_rule = seen_rules.get(rule).unwrap();
                let prev_rule = (*prev_rule).clone();
                return Err(ConfigValidationError::DuplicateRule(index, rule.clone(), prev_rule));
            }
            seen_rules.insert(rule);
        }
        Ok(())
    }

    pub fn validate (&self) -> Result<(), ConfigValidationError> {
        self.validate_default_preset()?;
        self.validate_rules()?;
        Ok(())
    }

    pub fn find_rule <T: Matchable> (&self, target: &T) -> Option<&Rule> {
        self.rules.iter().find(|rule| target.matches(&rule.on))
    }

    pub fn find_preset <T: Matchable> (&self, target: &T) -> (&str, &Preset) {
        let preset_name: &str = self.find_rule(target)
                .map(|rule| &rule.preset)
                .unwrap_or_else(|| &self.default_preset);

        let preset = self.get_preset(preset_name).unwrap();
        (preset_name, preset)
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
        presets.insert(
            String::from("None"),
            Preset {
                description: Some(String::from("Dont apply any affinity (use system defaults)")),
                ..Preset::default()
            },
        );
        let rules: Vec<Rule> = vec![
            Rule {
                on: Matcher::NoPath,
                preset: String::from("None"),
                description: Some(String::from("Matches all system processes and leaves them alone"))
            },
            Rule {
                on: Matcher::Path(PathBuf::from("C:\\Windows")),
                preset: String::from("None"),
                description: Some(String::from("Matches all windows processes and leaves them alone"))
            },
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