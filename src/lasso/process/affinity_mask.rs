use std::fmt;

use serde::{
    Serialize,
    Deserialize,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AffinityMask(pub usize);

impl AffinityMask {
    pub fn format (&self) -> String {
        match self.0 {
            0..=0xFFFF => format!("{:04X}", self.0),
            0x10000..=0xFFFFFFFF => format!("{:08X}", self.0),
            _ => format!("{:016X}", self.0),
        }
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