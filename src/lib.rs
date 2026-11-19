pub mod app;
pub mod education;
pub mod storage;
pub mod ui;

pub const APP_NAME: &str = "Education";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const APP_DESCRIPTION: &str =
    "A cross-platform educational application built with Rust.";

pub const APP_REPOSITORY: &str =
    "https://github.com/brenocooler98/Education";

pub const APP_AUTHOR: &str = "brenocooler98";

pub fn get_app_name() -> &'static str {
    APP_NAME
}

pub fn get_app_version() -> &'static str {
    APP_VERSION
}

pub fn get_app_description() -> &'static str {
    APP_DESCRIPTION
}

pub fn get_app_repository() -> &'static str {
    APP_REPOSITORY
}

pub fn is_android() -> bool {
    cfg!(target_os = "android")
}

pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}

pub fn is_mobile() -> bool {
    is_android()
}

pub fn is_desktop() -> bool {
    is_windows() || is_linux()
}

pub fn get_platform() -> &'static str {
    if is_android() {
        "Android"
    } else if is_windows() {
        "Windows"
    } else if is_linux() {
        "Linux"
    } else {
        "Unknown"
    }
}
