use std::error::Error;

use tauri::{App, WindowBuilder, WindowUrl};

pub fn build_new_window(app: &mut App) -> Result<(), Box<dyn Error>>{
    let a_window = WindowBuilder::new(
        app,
        "external",
        WindowUrl::External("https://tauri.app/".parse().unwrap())
    ).build();
    Ok(())
}