use serde::{
    Serialize,
    Deserialize,
};

#[derive(Clone, Debug, PartialEq)]
pub struct AffinityMask(pub u64);

impl Serialize for AffinityMask {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{:08X}", self.0))
    }
}

impl<'de> Deserialize<'de> for AffinityMask {
    fn deserialize<D>(deserializer: D) -> Result<AffinityMask, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let u = u64::from_str_radix(&s, 16).map_err(serde::de::Error::custom)?;
        Ok(AffinityMask(u))
    }
}