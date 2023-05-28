use crate::prelude::*;
use serde::{Deserialize, Serialize};

// #[serde(skip)] for fields that should not be serialized - like Atomic Reference Counted types

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct EyeTrackVR {}

impl EyeTrackVR {
    pub fn new() -> Self {
        Self {}.init_logger().unwrap()
    }

    pub fn run(&self) -> Result<()> {
        Ok(())
    }

    pub fn init_logger(self) -> Result<Self> {
        Logger::init(log::LevelFilter::max()).unwrap();
        Ok(self)
    }
}

/// below is a test to make sure the EyeTrackVR struct is safe to send across threads
#[allow(dead_code)]
fn is_normal<T>()
where
    T: Sized + Send + Sync + Unpin,
{
}

#[test]
fn normal_types() {
    is_normal::<EyeTrackVR>();
}
