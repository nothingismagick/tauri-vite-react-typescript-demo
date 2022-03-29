#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{window::WindowBuilder, AppHandle, WindowUrl};

/// Create the confetti overlay window
#[tauri::command]
fn open_overlay_window(app: AppHandle) -> Result<(), String> {
    let result = WindowBuilder::new(&app, "second", WindowUrl::App("/overlay".into()))
        .decorations(false)
        .transparent(true)
        .maximized(true)
        .build();

    match result {
        Ok(_) => {
            println!("Window created successfully!");
            Ok(())
        }
        Err(err) => {
            println!("Failed to create Window {}", err);
            Err("Failed to create Window".to_string())
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![open_overlay_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
