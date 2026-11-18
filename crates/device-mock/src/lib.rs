//! Explicitly simulated devices; never identifies or opens physical hardware.
use device_core::*;
use std::time::SystemTime;

pub struct MockDevice {
    name: String,
    battery: bool,
    connection: Connection,
    percentage: Option<Percentage>,
    charge: ChargeState,
}
impl MockDevice {
    pub fn mouse() -> Self {
        Self {
            name: "Mock SUPERLIGHT (simulated)".into(),
            battery: true,
            connection: Connection::Online,
            percentage: Percentage::new(72).ok(),
            charge: ChargeState::Discharging,
        }
    }
    pub fn keyboard() -> Self {
        Self {
            name: "Mock TAC75 HE (simulated)".into(),
            battery: false,
            connection: Connection::Online,
            percentage: None,
            charge: ChargeState::Unknown,
        }
    }
    pub fn set_connection(&mut self, value: Connection) {
        self.connection = value;
    }
    pub fn set_battery(&mut self, value: Option<Percentage>, charge: ChargeState) {
        self.percentage = value;
        self.charge = charge;
    }
}
impl DeviceAdapter for MockDevice {
    fn name(&self) -> &str {
        &self.name
    }
    fn capabilities(&self) -> Capabilities {
        Capabilities {
            battery: self.battery,
        }
    }
    fn connection(&self) -> Connection {
        self.connection
    }
    fn read_battery(&mut self) -> Result<BatteryReading, DeviceError> {
        if !self.battery {
            return Err(DeviceError::Unsupported);
        }
        if self.connection == Connection::Disconnected {
            return Err(DeviceError::Disconnected);
        }
        Ok(BatteryReading {
            percentage: self.percentage,
            charge: self.charge,
            observed_at: SystemTime::now(),
            stale: self.connection == Connection::Sleeping,
        })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wired_keyboard_has_no_battery() {
        let mut device = MockDevice::keyboard();
        assert!(!device.capabilities().battery);
        assert_eq!(device.read_battery(), Err(DeviceError::Unsupported));
    }
    #[test]
    fn unknown_is_distinct_from_zero() {
        let mut device = MockDevice::mouse();
        device.set_battery(None, ChargeState::Unknown);
        assert_eq!(device.read_battery().unwrap().percentage, None);
        device.set_battery(Some(Percentage::new(0).unwrap()), ChargeState::Discharging);
        assert_eq!(device.read_battery().unwrap().percentage.unwrap().get(), 0);
    }
    #[test]
    fn disconnect_sleep_and_reconnect() {
        let mut device = MockDevice::mouse();
        device.set_connection(Connection::Disconnected);
        assert_eq!(device.read_battery(), Err(DeviceError::Disconnected));
        device.set_connection(Connection::Sleeping);
        assert!(device.read_battery().unwrap().stale);
        device.set_connection(Connection::Online);
        assert!(!device.read_battery().unwrap().stale);
    }
    #[test]
    fn charging_is_preserved() {
        let mut device = MockDevice::mouse();
        device.set_battery(Some(Percentage::new(50).unwrap()), ChargeState::Charging);
        assert_eq!(device.read_battery().unwrap().charge, ChargeState::Charging);
    }
}
