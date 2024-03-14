use std::{fmt, ops::BitAnd};

use serde::{
    Serialize,
    Deserialize,
};

use crate::locals::NUM_CORES;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct AffinityMask(pub usize);

impl AffinityMask {
    pub fn format (&self) -> String {
        NUM_CORES.with(|n| match n {
            0..=4 => format!("{:01X}", self.0),
            5..=8 => format!("{:02X}", self.0),
            9..=16 => format!("{:04X}", self.0),
            17..=32 => format!("{:08X}", self.0),
            33..=64 => format!("{:016X}", self.0),
            65..=128 => format!("{:032X}", self.0),
            129..=256 => format!("{:064X}", self.0),
            _ => panic!("Too many cores. Nasa has been notified."),
        })
    }
}

impl BitAnd for AffinityMask {
    type Output = Self;

    fn bitand (self, rhs: Self) -> Self {
        AffinityMask(self.0 & rhs.0)
    }
}

impl From<usize> for AffinityMask {
    fn from (u: usize) -> Self {
        AffinityMask(u)
    }
}

impl Into<usize> for AffinityMask {
    fn into (self) -> usize {
        self.0
    }
}

impl fmt::Display for AffinityMask {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl Serialize for AffinityMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.format())
    }
}

impl<'de> Deserialize<'de> for AffinityMask {
    fn deserialize<D>(deserializer: D) -> Result<AffinityMask, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let u = usize::from_str_radix(&s, 16).map_err(serde::de::Error::custom)?;
        Ok(AffinityMask(u))
    }
}