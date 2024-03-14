use super::{Rule, Matchable};

pub struct RuleMatch<'r, 't, T: Matchable> {
    rule: &'r Rule,
    target: &'t T,
}

impl <'r, 't, T: Matchable> RuleMatch<'r, 't, T> {
    pub fn new (rule: &'r Rule, target: &'t T) -> RuleMatch<'r, 't, T> {
        RuleMatch {
            rule,
            target,
        }
    }
}