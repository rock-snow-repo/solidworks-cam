//! Windows registry read helpers for Solidworks Cam.

pub const APP_KEY: &str = "Software\\Solidworks\\Solidworks Cam";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Solidworks Cam";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Solidworks Cam") },
        RegValue { name: "Version",     value: String::from("v1.7.21") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Solidworks Cam") },
        RegValue { name: "Publisher",       value: String::from("Solidworks") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Solidworks Cam\\uninstall.exe\"") },
    ]
}
