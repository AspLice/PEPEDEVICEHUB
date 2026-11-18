//! Public metadata only: no paths, strings, serials, or input reports.
use serde_json::{Value, json};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Metadata {
    pub vendor_id: u16,
    pub product_id: u16,
    pub usage_page: u16,
    pub usage: u16,
    pub interface_number: i32,
    pub release_number: u16,
}
pub fn target(vid: u16, pid: u16) -> bool {
    matches!((vid, pid), (0x046d, 0xc547) | (0x3151, 0x502d))
}
pub fn report(mut devices: Vec<Metadata>) -> Value {
    devices.retain(|d| target(d.vendor_id, d.product_id));
    devices.sort_unstable();
    devices.dedup();
    let devices: Vec<_> = devices.into_iter().map(|d| json!({
        "vendor_id": format!("{:04X}",d.vendor_id),
        "product_id": format!("{:04X}",d.product_id),
        "usage_page": format!("{:04X}",d.usage_page),
        "usage": format!("{:04X}",d.usage),
        "interface_number": if d.interface_number < 0 { None } else { Some(d.interface_number) },
        "release_number_raw": format!("{:04X}",d.release_number),
    })).collect();
    json!({"schema_version":2,"source":"hidapi Windows native enumeration",
        "mode":"metadata-only","version":env!("CARGO_PKG_VERSION"),
        "identity_scope":"aggregated public metadata, not physical device identity",
        "devices":devices})
}
#[cfg(test)]
mod tests {
    use super::*;
    fn receiver() -> Metadata {
        Metadata {
            vendor_id: 0x046d,
            product_id: 0xc547,
            usage_page: 0xff00,
            usage: 1,
            interface_number: 2,
            release_number: 0x1234,
        }
    }
    #[test]
    fn scope_is_exact() {
        assert!(target(0x046d, 0xc547));
        assert!(target(0x3151, 0x502d));
        assert!(!target(0x046d, 0xc548));
        assert!(!target(0xffff, 0x502d));
    }
    #[test]
    fn whitelist_and_format() {
        let output = report(vec![receiver()]);
        let d = &output["devices"][0];
        assert_eq!(d["vendor_id"], "046D");
        assert_eq!(d["usage_page"], "FF00");
        assert_eq!(d["interface_number"], 2);
        assert_eq!(d["release_number_raw"], "1234");
        assert_eq!(d.as_object().unwrap().len(), 6);
        assert!(d.get("serial_number").is_none());
        assert!(d.get("path").is_none());
        assert!(d.get("product_name").is_none());
    }
    #[test]
    fn deduplicates_without_losing_usage() {
        let a = receiver();
        let mut b = a;
        b.usage = 2;
        assert_eq!(
            report(vec![a, a, b])["devices"].as_array().unwrap().len(),
            2
        );
    }
    #[test]
    fn excludes_unrelated_and_handles_empty() {
        let mut a = receiver();
        a.product_id = 0xffff;
        assert_eq!(report(vec![a])["devices"], json!([]));
        assert_eq!(report(vec![])["devices"], json!([]));
    }
    #[test]
    fn missing_interface_is_null() {
        let mut a = receiver();
        a.interface_number = -1;
        assert_eq!(
            report(vec![a])["devices"][0]["interface_number"],
            Value::Null
        );
    }
}
