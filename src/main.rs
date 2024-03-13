/* Prevents additional console window on Windows in release */
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#![allow(non_snake_case)]
pub mod ui;
pub mod system;
pub mod lasso;
pub mod macros;

use system::{create_config_store, create_process_manager, ConfigStoreError};
use ui::launch_app;

fn main() -> () {
    let mut config_store = create_config_store("yaml", "config.yaml");
    let config = match config_store.get() {
        Ok(config) => config,
        Err(ConfigStoreError::NotInitialized) => {
            let conf = lasso::Config::default();
            config_store.put(conf.clone()).unwrap();
            conf
        },
        Err(err) => panic!("Error reading config: {}", err),
    };    

    let process_manager = create_process_manager();
    process_manager.apply(&config).unwrap();

    // dispose config_manager, config, and process_manager
    drop(config_store);
    drop(config);
    drop(process_manager);

    launch_app();
}