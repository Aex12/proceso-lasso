pub mod windows;
pub mod config;

pub use config::create_config_store;

use windows::WindowsProcessManager;
use crate::lasso::ProcessManager;

pub fn create_process_manager () -> Box<dyn ProcessManager> {
    Box::new(WindowsProcessManager::new().unwrap())
}