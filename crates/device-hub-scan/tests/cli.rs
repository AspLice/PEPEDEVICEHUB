use std::process::Command;
#[test]
fn rejects_arguments() {
    assert_eq!(
        Command::new(env!("CARGO_BIN_EXE_device-hub-scan"))
            .arg("--write")
            .output()
            .unwrap()
            .status
            .code(),
        Some(2)
    );
}
#[cfg(windows)]
#[test]
fn native_enumeration_is_valid_json() {
    let output = Command::new(env!("CARGO_BIN_EXE_device-hub-scan"))
        .output()
        .unwrap();
    assert!(output.status.success());
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["schema_version"], 2);
    assert_eq!(report["mode"], "metadata-only");
    for d in report["devices"].as_array().unwrap() {
        assert_eq!(d.as_object().unwrap().len(), 6);
    }
}
#[cfg(not(windows))]
#[test]
fn unsupported_platform_fails_explicitly() {
    let output = Command::new(env!("CARGO_BIN_EXE_device-hub-scan"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}
