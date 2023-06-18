#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::generate_handler;

mod pashword;

fn main() {
    tauri::Builder::default()
        .invoke_handler(generate_handler![generate_pashword, sanitize_pashword])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command(async)]
fn generate_pashword(to_hash: &str, pashword_length: i32, website: &str, username: &str) -> String {
    return pashword::generate_pashword(to_hash, pashword_length, website, username);
}

#[tauri::command]
fn sanitize_pashword(pashword: &str, no_symbols: bool, no_numbers: bool) -> String {
    return pashword::sanitize(pashword, no_symbols, no_numbers);
}
