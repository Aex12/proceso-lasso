pub mod locals;
mod process;
mod preset;
mod rule;
mod config;

pub use process::{AffinityMask, Process, ProcessList, ProcessManager};
pub use preset::Preset;
pub use rule::{Matcher, Matchable, Rule};
pub use config::{Config, ConfigValidationError};