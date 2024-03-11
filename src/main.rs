/* Prevents additional console window on Windows in release */
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]
pub mod ui;
pub mod system;
pub mod lasso;

use lasso::ProcessManager;
use system::windows::WindowsProcessManager;
use system::config::create_config_store;
use ui::launch_app;

fn main() -> () {
    let mut config_store = create_config_store("yaml", "config.yaml");
    let config = config_store.get().unwrap();
    // put after get to ensure the file is created if it doesn't exist
    config_store.put(config.clone()).unwrap();

    let process_manager = WindowsProcessManager::new().unwrap();
    process_manager.apply(&config).unwrap();

    // dispose config_manager, config, and process_manager
    drop(config_store);
    drop(config);
    drop(process_manager);

    launch_app();
}