#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

//extern crate base64;
//extern crate leptess;

use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_image(url: &str) -> String {
    println!("Here!");
    let base64_data = url.trim_start_matches("data:image/png;base64,");
    let bytes = general_purpose::STANDARD.decode(base64_data).unwrap();
    format!(
        "Hello, {}! You've been greeted from Rust!",
        &base64_data[0..10]
    )
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
