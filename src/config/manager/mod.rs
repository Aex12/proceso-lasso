mod yaml;
mod memory;
pub use yaml::YamlConfigManager;
pub use memory::MemoryConfigManager;
use super::LassoConfig;

pub trait ConfigManager {
    fn get (&self) -> Result<LassoConfig, Box<dyn std::error::Error>>;
    fn put (&mut self, config: LassoConfig) -> Result<(), Box<dyn std::error::Error>>;
    fn load (&mut self) -> Result<(), Box<dyn std::error::Error>>;
}