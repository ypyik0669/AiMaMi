use serde_json::Value;

#[test]
fn tauri_javascript_and_rust_versions_share_major_and_minor() {
    let package: Value = serde_json::from_str(include_str!("../../package.json")).unwrap();
    let lock: toml::Value = toml::from_str(include_str!("../Cargo.lock")).unwrap();
    let manifest: toml::Value = toml::from_str(include_str!("../Cargo.toml")).unwrap();
    let crates = lock["package"].as_array().unwrap();

    for (npm, rust) in [
        ("@tauri-apps/api", "tauri"),
        ("@tauri-apps/plugin-dialog", "tauri-plugin-dialog"),
        ("@tauri-apps/plugin-process", "tauri-plugin-process"),
        ("@tauri-apps/plugin-shell", "tauri-plugin-shell"),
        ("@tauri-apps/plugin-updater", "tauri-plugin-updater"),
    ] {
        let js_version = package["dependencies"][npm].as_str().unwrap();
        let rust_version = crates
            .iter()
            .find(|entry| entry["name"].as_str() == Some(rust))
            .unwrap()["version"]
            .as_str()
            .unwrap();
        let js_minor: Vec<_> = js_version.trim_start_matches('^').split('.').take(2).collect();
        let rust_minor: Vec<_> = rust_version.split('.').take(2).collect();
        assert_eq!(js_minor, rust_minor, "{npm} {js_version} / {rust} {rust_version}");
        assert!(
            js_version.chars().all(|c| c.is_ascii_digit() || c == '.'),
            "{npm} must use an exact version to avoid minor-version drift"
        );
        let requirement = manifest["dependencies"][rust]
            .as_str()
            .or_else(|| manifest["dependencies"][rust]["version"].as_str())
            .unwrap();
        assert_eq!(
            requirement,
            format!("={rust_version}"),
            "{rust} must stay pinned to the version checked above"
        );
    }
}
