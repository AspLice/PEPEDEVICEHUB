use std::process::ExitCode;
fn main() -> ExitCode {
    if std::env::args_os().len() != 1 {
        eprintln!("Usage: device-hub-scan (no arguments; metadata only)");
        return ExitCode::from(2);
    }
    run()
}
#[cfg(windows)]
fn run() -> ExitCode {
    let api = match hidapi::HidApi::new() {
        Ok(api) => api,
        Err(_) => {
            // Provider errors may contain private device paths. Never print them.
            eprintln!("HID enumeration failed; no private details exported.");
            return ExitCode::FAILURE;
        }
    };
    let devices = api
        .device_list()
        .filter(|d| device_hub_scan::target(d.vendor_id(), d.product_id()))
        .map(|d| device_hub_scan::Metadata {
            vendor_id: d.vendor_id(),
            product_id: d.product_id(),
            usage_page: d.usage_page(),
            usage: d.usage(),
            interface_number: d.interface_number(),
            release_number: d.release_number(),
        })
        .collect();
    println!(
        "{}",
        serde_json::to_string_pretty(&device_hub_scan::report(devices))
            .expect("fixed metadata is serializable")
    );
    ExitCode::SUCCESS
}
#[cfg(not(windows))]
fn run() -> ExitCode {
    eprintln!("This metadata scanner requires Windows. No hardware was accessed.");
    ExitCode::from(2)
}
