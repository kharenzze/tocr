#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

//extern crate base64;
//extern crate leptess;
//
use base64::{engine::general_purpose, Engine};
use leptess::{leptonica, tesseract};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn process_image(url: &str) -> String {
    let base64_data = url.trim_start_matches("data:image/png;base64,");
    let bytes = general_purpose::STANDARD.decode(base64_data).unwrap();

    let mut api = tesseract::TessApi::new(None, "eng").unwrap();
    let pix = leptonica::pix_read_mem(&bytes).unwrap();
    api.set_image(&pix);

    let text = api.get_utf8_text();
    text.unwrap()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
