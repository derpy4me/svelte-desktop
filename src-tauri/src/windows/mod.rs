use std::error::Error;

use tauri::{AppHandle, WindowBuilder, WindowUrl};

pub fn build_new_window(app: &AppHandle) -> Result<(), Box<dyn Error>> {
    WindowBuilder::new(
        app,
        "external",
        WindowUrl::External("https://tauri.app/".parse().unwrap()),
    )
    .build()?;
    Ok(())
}
