mod affinity;
mod list;
mod manager;
mod process;
// mod token;

use list::getProcessList;
pub use manager::WindowsProcessManager;
pub use process::WindowsProcess;