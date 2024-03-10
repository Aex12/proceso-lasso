
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};
use super::Matcher;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rule {
    pub on: Matcher,
    pub preset: String,
    pub description: Option<String>,
}

impl PartialEq for Rule {
    fn eq (&self, other: &Self) -> bool {
        self.on == other.on
    }
}

impl Eq for Rule {
    fn assert_receiver_is_total_eq(&self) {
        // This is a no-op, but it's required because `Eq` is a marker trait.
    }
}

impl Hash for Rule {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.on.hash(state);
    }
}