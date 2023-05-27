use crate::prelude::*;

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
