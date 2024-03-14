use lib_core::AffinityMask;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct WindowsAffinity {
    process: AffinityMask,
    system: AffinityMask,
}

impl WindowsAffinity {
    pub fn new (process: AffinityMask, system: AffinityMask) -> Self {
        WindowsAffinity {
            process,
            system,
        }
    }

    pub fn process (&self) -> AffinityMask {
        self.process
    }

    pub fn system (&self) -> AffinityMask {
        self.system
    }
}