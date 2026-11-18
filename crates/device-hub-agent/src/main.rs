use device_core::DeviceAdapter;
use device_mock::MockDevice;
fn main() -> std::process::ExitCode {
    if std::env::args().skip(1).collect::<Vec<_>>() != ["--mock", "--once"] {
        eprintln!("Phase 0: only --mock --once is supported; no physical HID access.");
        return std::process::ExitCode::from(2);
    }
    println!("Device HUB — MOCK DATA ONLY");
    for mut device in [MockDevice::mouse(), MockDevice::keyboard()] {
        print!("{}: {:?}", device.name(), device.connection());
        if device.capabilities().battery {
            match device.read_battery() {
                Ok(reading) => print!(
                    " battery={:?}% {:?}",
                    reading.percentage.map(|p| p.get()),
                    reading.charge
                ),
                Err(error) => print!(" battery error={error:?}"),
            }
        } else {
            print!(" USB / battery unsupported");
        }
        println!();
    }
    std::process::ExitCode::SUCCESS
}
