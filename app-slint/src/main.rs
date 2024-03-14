slint::include_modules!();

use lib_system::{create_config_store, create_process_manager};

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut config_store = create_config_store("yaml", "config.yaml");
    let config = config_store.initialize().unwrap();

    let process_manager = create_process_manager();

    let ui = AppWindow::new()?;

    ui.set_counter(0);

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.on_request_decrease_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() - 1);
        }
    });

    ui.run()?;
    // slint::run_event_loop_until_quit()?;
    Ok(())
}
