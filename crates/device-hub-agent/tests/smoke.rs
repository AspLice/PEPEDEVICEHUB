use std::process::Command;
#[test]
fn mock_snapshot_is_explicit_and_keyboard_has_no_percentage() {
    let result = Command::new(env!("CARGO_BIN_EXE_device-hub-agent"))
        .args(["--mock", "--once"])
        .output()
        .unwrap();
    assert!(result.status.success());
    let text = String::from_utf8(result.stdout).unwrap();
    assert!(text.contains("MOCK DATA ONLY"));
    assert!(text.contains("Some(72)%"));
    let keyboard = text.lines().find(|line| line.contains("TAC75")).unwrap();
    assert!(keyboard.contains("battery unsupported"));
    assert!(!keyboard.contains('%'));
}
#[test]
fn real_hardware_mode_is_rejected() {
    assert_eq!(
        Command::new(env!("CARGO_BIN_EXE_device-hub-agent"))
            .output()
            .unwrap()
            .status
            .code(),
        Some(2)
    );
}
