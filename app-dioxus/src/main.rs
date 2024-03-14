/* Prevents additional console window on Windows in release */
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case)]
pub mod components;
pub mod app;

use app::launch_app;
use lib_system::create_config_store;

fn main() -> () {
    let mut config_store = create_config_store("yaml", "config.yaml");
    config_store.initialize().unwrap();
    launch_app();
}