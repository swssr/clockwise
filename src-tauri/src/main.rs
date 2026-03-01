// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod fns;
mod tray;

use chrono::Utc;
use chrono_tz::Tz;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::init,
            command::show_menubar_panel
        ])
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let tray = tray::create(app.app_handle())?;

           let tray_handle = tray.clone();

           let tz_austin: Tz = "America/Chicago".parse().unwrap();
            let tz_ny: Tz = "America/New_York".parse().unwrap();
            let tz_manila: Tz = "Asia/Manila".parse().unwrap();
                            
            tauri::async_runtime::spawn(async move {
                loop {
                    let now = Utc::now();

                    let austin = now.with_timezone(&tz_austin).format("%H:%M");
                    let ny = now.with_timezone(&tz_ny).format("%H:%M");
                    let manila = now.with_timezone(&tz_manila).format("%H:%M");

                    let title = format!("TX {} | NY {} | MNL {}", austin, ny, manila);
                    let _ = tray_handle.set_title(Some(&title));

                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
