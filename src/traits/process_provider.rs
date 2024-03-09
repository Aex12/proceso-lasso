use crate::structs::Process;

pub trait ProcessProvider {
    fn getProcs () -> Process;
}