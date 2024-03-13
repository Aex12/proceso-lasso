use windows::Win32::{
    Foundation::{CloseHandle, HANDLE}, 
    System::Threading::{GetProcessAffinityMask, OpenProcess, SetProcessAffinityMask, PROCESS_QUERY_INFORMATION, PROCESS_SET_INFORMATION},
};

use crate::lasso::{AffinityMask, Process};
use crate::macros::debug_println;

use super::affinity::WindowsAffinity;

#[derive(Debug)]
pub struct WindowsProcess {
    pid: i32,
}

impl WindowsProcess {
    pub fn open (pid: i32) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(WindowsProcess {
            pid: pid,
        })
    }

    /**
     * Set the affinity of the process to the given affinity mask.
     * Returns a tuple that contains:
     *  0. previous affinity mask of the process.
     *  1. new affinity mask of the process.
     */
    pub fn set_affinity (&self, affinity: AffinityMask) -> Result<(AffinityMask, AffinityMask), Box<dyn std::error::Error>> {
        let curr_affinity = self.get_affinity()?;
        let new_affinity = affinity & curr_affinity.system();
        debug_println!("[{}] Curr: {}, Sys: {} Req: {}. New: {}. ", self.pid, curr_affinity.process(), curr_affinity.system(), affinity, AffinityMask::from(new_affinity));
        if new_affinity == curr_affinity.process() {
            return Ok((curr_affinity.process(), new_affinity.into()));
        }
        unsafe {
            let handle: HANDLE = OpenProcess(PROCESS_SET_INFORMATION, false, self.pid.try_into()?)?;
            SetProcessAffinityMask(handle, new_affinity.into())?;
            CloseHandle(handle)?;
        }
        Ok((curr_affinity.process(), new_affinity.into()))
    }

    pub fn get_affinity (&self) -> Result<WindowsAffinity, Box<dyn std::error::Error>> {
        let mut process_affinity: usize = 0;
        let mut system_affinity: usize = 0;
        unsafe {
            let handle: HANDLE = OpenProcess(PROCESS_QUERY_INFORMATION, false, self.pid.try_into()?)?;
            GetProcessAffinityMask(handle, &mut process_affinity as *mut usize, &mut system_affinity as *mut usize)?;
            CloseHandle(handle)?;
        }
        Ok(WindowsAffinity::new(process_affinity.into(), system_affinity.into()))
    }
}

impl From<Process> for WindowsProcess {
    fn from (process: Process) -> Self {
        WindowsProcess::open(process.pid).unwrap()
    }
}

impl Into<AffinityMask> for WindowsProcess {
    fn into (self) -> AffinityMask {
        self.get_affinity().unwrap().process()
    }
}

impl PartialEq for WindowsProcess {
    fn eq (&self, other: &Self) -> bool {
        self.pid == other.pid
    }
}

impl Eq for WindowsProcess {}