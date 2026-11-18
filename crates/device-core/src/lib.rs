//! Hardware-independent domain model. No device I/O is performed here.
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Percentage(u8);
impl Percentage {
    pub fn new(value: u8) -> Result<Self, DeviceError> {
        if value <= 100 {
            Ok(Self(value))
        } else {
            Err(DeviceError::InvalidPercentage)
        }
    }
    pub fn get(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connection {
    Online,
    Sleeping,
    Disconnected,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChargeState {
    Discharging,
    Charging,
    Full,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryReading {
    pub percentage: Option<Percentage>,
    pub charge: ChargeState,
    pub observed_at: SystemTime,
    pub stale: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities {
    pub battery: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceError {
    Disconnected,
    Unsupported,
    InvalidPercentage,
    Timeout,
}

/// Agent owns adapters. Write operations are intentionally absent in Phase 0.
pub trait DeviceAdapter {
    fn name(&self) -> &str;
    fn capabilities(&self) -> Capabilities;
    fn connection(&self) -> Connection;
    fn read_battery(&mut self) -> Result<BatteryReading, DeviceError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn percentage_boundaries() {
        assert_eq!(Percentage::new(0).unwrap().get(), 0);
        assert_eq!(Percentage::new(100).unwrap().get(), 100);
        for n in 101..=255 {
            assert_eq!(Percentage::new(n), Err(DeviceError::InvalidPercentage));
        }
    }
}
