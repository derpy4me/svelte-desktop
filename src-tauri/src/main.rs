#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod requests;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_person() -> requests::FakeResponse {
    match requests::request_test() {
        Ok(response) => response,
        Err(error) => {
            println!("{}", error);
            requests::FakeResponse::default()
        },
    }

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,get_person])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
