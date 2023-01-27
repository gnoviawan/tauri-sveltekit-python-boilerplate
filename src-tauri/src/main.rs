#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::api::process::Command;
use tauri::api::process::CommandEvent;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder
        ::default()
        .setup(|app| {
            tauri::async_runtime::spawn(async move {
                let (mut rx, mut child) = Command::new_sidecar("api")
                    .expect("failed to setup `app` sidecar")
                    .spawn()
                    .expect("Failed to spawn packaged node");
                //
                let mut i = 0;

                while let Some(event) = rx.recv().await {
                    if let CommandEvent::Stdout(line) = event {
                        println!("{}", line);

                        if i == 4 {
                            child.write("message from Rust\n".as_bytes()).unwrap();
                            i = 0;
                        }
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}