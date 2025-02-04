use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

use crate::AppEntry;

pub fn get_available_apps() -> HashMap<String, AppEntry> {
    let mut apps: HashMap<String, AppEntry> = HashMap::new();

    // apps.extend(get_installed_apps().into_iter());  // Not working properly, this is returning some wrong EXE files
    apps.extend(get_startup_apps().into_iter());

    apps
}

pub fn get_startup_apps() -> HashMap<String, AppEntry> {
    let mut apps: HashMap<String, AppEntry> = HashMap::new();
    let startup_app_paths = [
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu\\Programs",
        &format!(
            "{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs",
            dirs::home_dir().unwrap_or_default().to_string_lossy()
        ),
    ];

    for path in &startup_app_paths {
        let dir = Path::new(path);
        read_lnk_files(dir).iter().for_each(|p| {
            apps.insert(p.id.clone(), p.clone());
        });
    }

    apps
}

fn read_lnk_files(dir: &Path) -> Vec<AppEntry> {
    let mut apps: Vec<AppEntry> = vec![];
    if dir.exists() && dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    apps.extend(read_lnk_files(&path));
                } else {
                    if path.extension().map(|e| e == "lnk").unwrap_or(false) {
                        if let Some(app_name) = path.file_stem() {
                            let id = app_name.to_string_lossy().into_owned();
                            let result = lnk::ShellLink::open(path);

                            if let Ok(app_data) = result {
                                if let Some(info) = app_data.link_info().as_ref() {
                                    let location = info.local_base_path();
                                    let icon = app_data.icon_location();
                                    let keywords = app_data.name();
                                    if let Some(exec) = location {
                                        let app = AppEntry::new(
                                            id.clone(),
                                            id,
                                            exec.clone(),
                                            icon.clone(),
                                            keywords.clone(),
                                        );

                                        apps.push(app);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    apps
}

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
