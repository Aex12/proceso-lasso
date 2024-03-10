mod yaml;
mod memory;
pub use yaml::YamlConfigManager;
pub use memory::MemoryConfigManager;
use super::Config;

pub trait ConfigManager {
    fn get (&self) -> Result<Config, Box<dyn std::error::Error>>;
    fn put (&mut self, config: Config) -> Result<(), Box<dyn std::error::Error>>;
    fn load (&mut self) -> Result<(), Box<dyn std::error::Error>>;
}