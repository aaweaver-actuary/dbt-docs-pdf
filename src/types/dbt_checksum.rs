use derive_builder::Builder;
use std::fmt;

#[derive(Debug, Clone, Builder)]
pub struct DbtChecksum {
    name: DbtChecksumType,
    checksum: String,
}

impl DbtChecksum {
    pub fn new(name: DbtChecksumType, checksum: String) -> Self {
        DbtChecksum { name, checksum }
    }

    pub fn builder() -> DbtChecksumBuilder {
        DbtChecksumBuilder::default()
    }
}

#[derive(Debug, Clone)]
pub enum DbtChecksumType {
    Md5,
    Sha1,
    Sha256,
    Sha512,
}

impl fmt::Display for DbtChecksumType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbtChecksumType::Md5 => write!(f, "md5"),
            DbtChecksumType::Sha1 => write!(f, "sha1"),
            DbtChecksumType::Sha256 => write!(f, "sha256"),
            DbtChecksumType::Sha512 => write!(f, "sha512"),
        }
    }
}
