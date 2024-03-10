use windows::Win32::System::Threading::{OpenProcess, SetProcessAffinityMask};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::PROCESS_SET_INFORMATION;
use std::convert::TryInto;

use crate::lasso::AffinityMask;


pub(super) fn setProcessAffinity (pid: i32, affinity: &AffinityMask) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let process_handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid.try_into().unwrap())?;
        SetProcessAffinityMask(process_handle, affinity.0)?;
        CloseHandle(process_handle)?;
    }
    Ok(())
}