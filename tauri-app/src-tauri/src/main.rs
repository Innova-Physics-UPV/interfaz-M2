// This hides console window in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    //Logic in lib.rs
    tauri_app_lib::run();
}