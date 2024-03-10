
use serde::{Deserialize, Serialize};
use super::Matcher;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rule {
    pub on: Matcher,
    pub preset: String,
    pub description: Option<String>,
}

pub struct ProcessRuleMatch(pub Rule, pub crate::process::Process);