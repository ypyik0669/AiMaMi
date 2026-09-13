use crate::core::models::UpdateInstallabilityPayload;
#[cfg(target_os = "windows")]
use std::ffi::OsString;
use std::path::Path;

pub fn has_updater_config(config: Option<&serde_json::Value>) -> bool {
    config
        .and_then(|value| serde_json::from_value::<tauri_plugin_updater::Config>(value.clone()).ok())
        .is_some_and(|config| !config.pubkey.trim().is_empty() && !config.endpoints.is_empty())
}

#[cfg(target_os = "windows")]
pub fn windows_current_install_dir_arg() -> Option<OsString> {
    let exe = std::env::current_exe().ok()?;
    windows_install_dir_arg_from_exe(&exe)
}

#[cfg(target_os = "windows")]
fn windows_install_dir_arg_from_exe(exe: &Path) -> Option<OsString> {
    let install_dir = exe.parent()?;
    // NSIS reads /D=<dir> before .onInit and uses it as $INSTDIR. It must be
    // the final installer argument; the updater plugin appends this value after
    // its own /UPDATE /ARGS values, so Windows updates stay in-place even when
    // the user originally installed AiMaMi into a custom directory.
    Some(OsString::from(format!(
        "/D={}",
        install_dir.to_string_lossy()
    )))
}

pub fn check_update_installability() -> UpdateInstallabilityPayload {
    #[cfg(target_os = "macos")]
    {
        macos_update_installability()
    }

    #[cfg(not(target_os = "macos"))]
    {
        UpdateInstallabilityPayload {
            can_install: true,
            code: "ok".to_string(),
            executable_path: std::env::current_exe()
                .ok()
                .map(|path| path.to_string_lossy().to_string()),
            bundle_path: None,
            translocated: false,
            quarantined: false,
        }
    }
}

#[cfg(target_os = "macos")]
fn macos_update_installability() -> UpdateInstallabilityPayload {
    let executable_path = std::env::current_exe().ok();
    let bundle_path = executable_path
        .as_deref()
        .and_then(resolve_app_bundle_from_exe)
        .map(Path::to_path_buf);

    let translocated = executable_path
        .as_deref()
        .is_some_and(is_app_translocation_path)
        || bundle_path
            .as_deref()
            .is_some_and(is_app_translocation_path);
    let disk_image = executable_path
        .as_deref()
        .is_some_and(is_disk_image_mount_path)
        || bundle_path.as_deref().is_some_and(is_disk_image_mount_path);

    let quarantined = bundle_path.as_deref().is_some_and(has_quarantine_attribute);

    let code = if translocated {
        "app_translocation"
    } else if disk_image {
        "read_only_location"
    } else {
        "ok"
    };

    UpdateInstallabilityPayload {
        can_install: !(translocated || disk_image),
        code: code.to_string(),
        executable_path: executable_path.map(|path| path.to_string_lossy().to_string()),
        bundle_path: bundle_path.map(|path| path.to_string_lossy().to_string()),
        translocated,
        quarantined,
    }
}

#[cfg(target_os = "macos")]
fn resolve_app_bundle_from_exe(executable_path: &Path) -> Option<&Path> {
    executable_path
        .parent()
        .and_then(|path| path.parent())
        .and_then(|path| path.parent())
        .filter(|path| path.extension().is_some_and(|ext| ext == "app"))
}

#[cfg(target_os = "macos")]
fn is_app_translocation_path(path: &Path) -> bool {
    path.to_string_lossy().contains("/AppTranslocation/")
}

#[cfg(target_os = "macos")]
fn is_disk_image_mount_path(path: &Path) -> bool {
    path.to_string_lossy().starts_with("/Volumes/")
}

#[cfg(target_os = "macos")]
fn has_quarantine_attribute(path: &Path) -> bool {
    std::process::Command::new("xattr")
        .arg("-p")
        .arg("com.apple.quarantine")
        .arg(path)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod updater_tests {
    use super::has_updater_config;
    use serde_json::json;

    #[test]
    fn source_build_without_updater_config_does_not_enable_plugin() {
        let config: serde_json::Value =
            serde_json::from_str(include_str!("../../tauri.conf.json")).unwrap();
        assert!(!has_updater_config(config["plugins"].get("updater")));
        assert!(!has_updater_config(Some(&serde_json::Value::Null)));
    }

    #[test]
    fn incomplete_updater_config_does_not_enable_plugin() {
        for value in [
            json!({}),
            json!({"pubkey": "", "endpoints": ["https://updates.example.invalid/latest.json"]}),
            json!({"pubkey": "test-public-key", "endpoints": []}),
            json!({"pubkey": "test-public-key", "endpoints": ["not a URL"]}),
        ] {
            assert!(!has_updater_config(Some(&value)));
        }
    }

    #[test]
    fn configured_release_keeps_updater_enabled() {
        let value = json!({
            "pubkey": "test-public-key",
            "endpoints": ["https://updates.example.invalid/latest.json"]
        });
        assert!(has_updater_config(Some(&value)));
    }
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::{is_app_translocation_path, is_disk_image_mount_path, resolve_app_bundle_from_exe};
    use std::path::Path;

    #[test]
    fn resolves_bundle_from_macos_executable_path() {
        let exe = Path::new("/Applications/AiMaMi.app/Contents/MacOS/AiMaMi");
        let bundle = resolve_app_bundle_from_exe(exe).expect("bundle path");
        assert_eq!(bundle, Path::new("/Applications/AiMaMi.app"));
    }

    #[test]
    fn detects_app_translocation_paths() {
        let translocated = Path::new(
            "/private/var/folders/x/AppTranslocation/ABCDE/d/AiMaMi.app/Contents/MacOS/AiMaMi",
        );
        let regular = Path::new("/Applications/AiMaMi.app/Contents/MacOS/AiMaMi");
        assert!(is_app_translocation_path(translocated));
        assert!(!is_app_translocation_path(regular));
    }

    #[test]
    fn detects_disk_image_mount_paths() {
        let dmg = Path::new("/Volumes/AiMaMi/AiMaMi.app/Contents/MacOS/AiMaMi");
        let regular = Path::new("/Applications/AiMaMi.app/Contents/MacOS/AiMaMi");
        assert!(is_disk_image_mount_path(dmg));
        assert!(!is_disk_image_mount_path(regular));
    }
}
