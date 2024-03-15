slint::include_modules!();

use std::rc::Rc;

use lib_core::ProcessManager;
use lib_system::{create_config_store, create_process_manager};
use slint::{StandardListViewItem, VecModel};

fn update_processes (ui: &AppWindow, process_manager: Box<dyn ProcessManager>) -> Result<(), Box<dyn std::error::Error>> {
    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());
    let process_list = process_manager.get_process_list()?;
    let search = ui.get_search().to_string();

    println!("Search: {}", search);

    for process in process_list.into_iter().filter(|p| p.name.contains(&search)) {
        let items = Rc::new(VecModel::default());
        let pid = StandardListViewItem::from(slint::format!("{}", process.pid));
        let name = StandardListViewItem::from(slint::format!("{}", process.name));
        let priority = StandardListViewItem::from(slint::format!("{}", process.priority));
        let path = StandardListViewItem::from(slint::format!("{}", process.path_str()));

        items.push(pid.into());
        items.push(name.into());
        items.push(priority.into());
        items.push(path.into());

        row_data.push(items.clone().into());
    }
    ui.set_processes(row_data.into());
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let mut config_store = create_config_store("yaml", "config.yaml");
    let config = config_store.initialize()?;

    let process_manager = create_process_manager();
    process_manager.apply(&config)?;

    let ui = AppWindow::new()?;

    update_processes(&ui, process_manager)?;

    ui.on_request_reload({
        let ui_handle = ui.as_weak();
        move || {
            update_processes(&ui_handle.unwrap(), create_process_manager()).unwrap();
        }
    });

    ui.on_search_change({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            update_processes(&ui, create_process_manager()).unwrap();
        }
    });

    ui.run()?;
    // slint::run_event_loop_until_quit()?;
    Ok(())
}
