// https://github.com/h4rldev/check_elevation/blob/master/src/lib.rs
use crate::lasso::SudoManager;

pub struct WindowsSudoManager;

impl WindowsSudoManager {
    pub fn new () -> Self {
        WindowsSudoManager
    }
}

impl SudoManager for WindowsSudoManager {
    fn is_elevated (&self) -> bool {
        todo!()
    }

    fn elevate (&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}