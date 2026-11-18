//! Offline HID++ 2.0 codec. No transport or device I/O is provided.
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    ReportId,
    Length,
    SoftwareId,
    Timeout,
    Budget,
    UnsupportedFeature,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frame {
    pub device: u8,
    pub feature: u8,
    pub function_software: u8,
    pub parameters: Vec<u8>,
}
impl Frame {
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        let size = match bytes.first() {
            Some(0x10) => 7,
            Some(0x11) => 20,
            _ => return Err(Error::ReportId),
        };
        if bytes.len() != size {
            return Err(Error::Length);
        }
        Ok(Self {
            device: bytes[1],
            feature: bytes[2],
            function_software: bytes[3],
            parameters: bytes[4..].to_vec(),
        })
    }
}

/// Only Root.GetFeature is constructed here. Feature indices are never guessed.
#[derive(Debug, Clone)]
pub struct DiscoveryRequest {
    bytes: [u8; 7],
}
impl DiscoveryRequest {
    pub fn new(device: u8, software_id: u8, feature_id: u16) -> Result<Self, Error> {
        if !(1..=15).contains(&software_id) {
            return Err(Error::SoftwareId);
        }
        let [high, low] = feature_id.to_be_bytes();
        Ok(Self {
            bytes: [0x10, device, 0, software_id, high, low, 0],
        })
    }
    pub fn bytes(&self) -> &[u8; 7] {
        &self.bytes
    }
    fn classify(&self, frame: Frame) -> Outcome {
        if frame.device != self.bytes[1] {
            return Outcome::Unrelated;
        }
        if frame.feature == 0xff
            && frame.function_software == self.bytes[2]
            && frame.parameters[0] == self.bytes[3]
        {
            return Outcome::DeviceError(frame.parameters[1]);
        }
        // 0x8f is a HID++ 1.0 error, not a normal Root reply.
        if frame.feature == 0x8f
            && frame.function_software == self.bytes[2]
            && frame.parameters[0] == self.bytes[3]
        {
            return Outcome::LegacyError(frame.parameters[1]);
        }
        if frame.function_software & 0x0f == 0 {
            return Outcome::Notification(frame);
        }
        if frame.feature == self.bytes[2] && frame.function_software == self.bytes[3] {
            return Outcome::Reply(frame);
        }
        Outcome::Unrelated
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    Reply(Frame),
    DeviceError(u8),
    LegacyError(u8),
    Notification(Frame),
    Unrelated,
}

/// One transaction, absolute monotonic deadline and a finite frame budget.
/// The caller must also bound its transport read to `remaining`.
pub struct PendingDiscovery {
    request: DiscoveryRequest,
    started: Instant,
    timeout: Duration,
    frames_left: usize,
}
impl PendingDiscovery {
    pub fn new(
        request: DiscoveryRequest,
        started: Instant,
        timeout: Duration,
        max_frames: usize,
    ) -> Self {
        Self {
            request,
            started,
            timeout,
            frames_left: max_frames,
        }
    }
    pub fn remaining(&self, now: Instant) -> Result<Duration, Error> {
        let elapsed = now.saturating_duration_since(self.started);
        if elapsed >= self.timeout {
            return Err(Error::Timeout);
        }
        if self.frames_left == 0 {
            return Err(Error::Budget);
        }
        Ok(self.timeout - elapsed)
    }
    pub fn receive(&mut self, bytes: &[u8], now: Instant) -> Result<Outcome, Error> {
        self.remaining(now)?;
        self.frames_left -= 1;
        Ok(self.request.classify(Frame::parse(bytes)?))
    }
}

/// Extract the dynamically returned index; zero means absent, never index zero.
pub fn discovered_index(frame: &Frame) -> Result<u8, Error> {
    match frame.parameters.first() {
        Some(index @ 1..=254) => Ok(*index),
        _ => Err(Error::UnsupportedFeature),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn pending(now: Instant) -> PendingDiscovery {
        PendingDiscovery::new(
            DiscoveryRequest::new(1, 8, 0x1004).unwrap(),
            now,
            Duration::from_millis(100),
            3,
        )
    }
    #[test]
    fn root_feature_big_endian() {
        assert_eq!(
            DiscoveryRequest::new(1, 8, 0x1004).unwrap().bytes(),
            &[0x10, 1, 0, 8, 0x10, 4, 0]
        );
        assert_eq!(
            DiscoveryRequest::new(1, 8, 0x1000).unwrap().bytes()[4..6],
            [0x10, 0]
        );
    }
    #[test]
    fn reject_reserved_and_out_of_range_software_id() {
        for id in [0, 16, 255] {
            assert_eq!(
                DiscoveryRequest::new(1, id, 0x1004).unwrap_err(),
                Error::SoftwareId
            );
        }
    }
    #[test]
    fn exact_report_sizes() {
        for n in 0..=21 {
            let mut data = vec![0; n];
            if n > 0 {
                data[0] = 0x10;
            }
            assert_eq!(Frame::parse(&data).is_ok(), n == 7);
            if n > 0 {
                data[0] = 0x11;
            }
            assert_eq!(Frame::parse(&data).is_ok(), n == 20);
        }
        assert_eq!(Frame::parse(&[0x12; 7]), Err(Error::ReportId));
    }
    #[test]
    fn matches_device_function_and_software() {
        let now = Instant::now();
        for data in [
            [0x10, 2, 0, 8, 5, 0, 0],
            [0x10, 1, 0, 9, 5, 0, 0],
            [0x10, 1, 0, 0x18, 5, 0, 0],
        ] {
            assert_eq!(pending(now).receive(&data, now), Ok(Outcome::Unrelated));
        }
        let Outcome::Reply(frame) = pending(now)
            .receive(&[0x10, 1, 0, 8, 5, 0, 0], now)
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(discovered_index(&frame), Ok(5));
    }
    #[test]
    fn long_response_supported() {
        let now = Instant::now();
        let mut data = [0; 20];
        data[..5].copy_from_slice(&[0x11, 1, 0, 8, 17]);
        assert!(matches!(
            pending(now).receive(&data, now),
            Ok(Outcome::Reply(_))
        ));
    }
    #[test]
    fn notifications_do_not_match() {
        let now = Instant::now();
        assert!(matches!(
            pending(now).receive(&[0x10, 1, 0, 0, 8, 0, 0], now),
            Ok(Outcome::Notification(_))
        ));
    }
    #[test]
    fn feature_and_legacy_errors_are_correlated() {
        let now = Instant::now();
        assert_eq!(
            pending(now).receive(&[0x10, 1, 0xff, 0, 8, 9, 0], now),
            Ok(Outcome::DeviceError(9))
        );
        assert_eq!(
            pending(now).receive(&[0x10, 1, 0x8f, 0, 8, 1, 0], now),
            Ok(Outcome::LegacyError(1))
        );
        assert!(!matches!(
            pending(now).receive(&[0x10, 2, 0xff, 0, 8, 9, 0], now),
            Ok(Outcome::DeviceError(_))
        ));
    }
    #[test]
    fn noise_never_extends_deadline() {
        let start = Instant::now();
        let mut p = pending(start);
        p.receive(&[0x10, 2, 0, 8, 0, 0, 0], start + Duration::from_millis(99))
            .unwrap();
        assert_eq!(
            p.remaining(start + Duration::from_millis(100)),
            Err(Error::Timeout)
        );
    }
    #[test]
    fn frame_budget_bounds_notification_flood() {
        let now = Instant::now();
        let mut p = pending(now);
        for _ in 0..3 {
            p.receive(&[0x10, 1, 2, 0, 0, 0, 0], now).unwrap();
        }
        assert_eq!(
            p.receive(&[0x10, 1, 0, 8, 5, 0, 0], now),
            Err(Error::Budget)
        );
    }
    #[test]
    fn malformed_frames_also_consume_budget() {
        let now = Instant::now();
        let mut p = pending(now);
        for _ in 0..3 {
            assert_eq!(p.receive(&[], now), Err(Error::ReportId));
        }
        assert_eq!(p.receive(&[], now), Err(Error::Budget));
    }
    #[test]
    fn missing_feature_is_not_root() {
        for index in [0, 255] {
            let frame = Frame::parse(&[0x10, 1, 0, 8, index, 0, 0]).unwrap();
            assert_eq!(discovered_index(&frame), Err(Error::UnsupportedFeature));
        }
    }
    #[test]
    fn zero_timeout_and_budget_stop_immediately() {
        let now = Instant::now();
        let req = DiscoveryRequest::new(1, 8, 0x1004).unwrap();
        assert_eq!(
            PendingDiscovery::new(req.clone(), now, Duration::ZERO, 3).remaining(now),
            Err(Error::Timeout)
        );
        assert_eq!(
            PendingDiscovery::new(req, now, Duration::from_secs(1), 0).remaining(now),
            Err(Error::Budget)
        );
    }
}
