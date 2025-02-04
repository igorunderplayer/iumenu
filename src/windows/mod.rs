use std::{collections::HashMap, ffi::OsStr, fs, path::Path};

use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

use crate::AppEntry;

pub fn get_installed_apps() -> HashMap<String, AppEntry> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let uninstall_paths = [
        "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
        "SOFTWARE\\Wow6432Node\\Microsoft\\Windows\\CurrentVersion\\Uninstall",
    ];

    let mut apps = HashMap::new();

    for path in &uninstall_paths {
        if let Ok(uninstall_key) = hklm.open_subkey(path) {
            for app in uninstall_key.enum_keys().filter_map(|k| k.ok()) {
                if let Ok(subkey) = uninstall_key.open_subkey(&app) {
                    let display_name: Result<String, _> = subkey.get_value("DisplayName");
                    let install_location: Result<String, _> = subkey.get_value("InstallLocation");
                    let display_icon: Result<String, _> = subkey.get_value("DisplayIcon");
                    let comments = subkey.get_value("Comments");

                    let icon = if let Ok(icon) = display_icon {
                        Some(icon)
                    } else {
                        None
                    };

                    let keywords = if let Ok(comments) = comments {
                        Some(comments)
                    } else {
                        None
                    };

                    if let (Ok(name), Ok(location)) = (display_name, install_location) {
                        if let Some(executable) = find_executable(&location) {
                            apps.insert(
                                name.clone(),
                                AppEntry::new(name.clone(), name, executable, icon, keywords),
                            );
                        }
                    }
                }
            }
        }
    }
    apps
}

fn find_executable(install_location: &str) -> Option<String> {
    let path = Path::new(install_location);
    if path.is_dir() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Some(ext) = entry_path.extension() {
                        if ext == "exe"
                            && !entry_path
                                .file_name()
                                .unwrap_or(OsStr::new(""))
                                .to_string_lossy()
                                .contains("uninstall")
                        {
                            return Some(entry_path.to_string_lossy().into_owned());
                        }
                    }
                }
            }
        }
    }
    None
}
