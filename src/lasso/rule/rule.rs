
use std::{fmt::Display, hash::{Hash, Hasher}};

use serde::{Deserialize, Serialize};
use super::{Matchable, Matcher, RuleMatch};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub on: Matcher,
    pub preset: String,
    pub description: Option<String>,
}

impl Rule {
    pub fn new (on: Matcher, preset: String, description: Option<String>) -> Rule {
        Rule {
            on,
            preset,
            description,
        }
    }

    #[inline]
    pub fn matches <T: Matchable> (&self, target: &T) -> bool {
        target.matches(&self.on)
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl PartialEq for Rule {
    fn eq (&self, other: &Self) -> bool {
        self.on == other.on
    }
}

impl Eq for Rule {}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.on.hash(state);
    }
}