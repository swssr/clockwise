// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod command;
mod fns;
mod tray;

use std::sync::Mutex;

use chrono::Utc;
use chrono_tz::Tz;
use chrono_tz::TZ_VARIANTS;

use tauri::Manager;

#[derive(Default)]
struct SelectedTimezones(Mutex<Vec<Tz>>);

#[tauri::command]
fn list_timezones() -> Vec<String> {
    TZ_VARIANTS.iter().map(|tz| tz.name().to_string()).collect()
}

#[tauri::command]
fn set_selected_timezones(
    state: tauri::State<'_, SelectedTimezones>,
    tz_names: Vec<String>,
) -> Result<(), String> {
    let mut parsed: Vec<Tz> = Vec::with_capacity(tz_names.len());

    for name in tz_names {
        let tz: Tz = name
            .parse()
            .map_err(|_| format!("Invalid timezone: {name}"))?;
        parsed.push(tz);
    }

    *state.0.lock().unwrap() = parsed;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command::init,
            command::show_menubar_panel,
            list_timezones,
            set_selected_timezones
        ])
        .manage(SelectedTimezones::default())
        .plugin(tauri_nspanel::init())
        .setup(|app| {
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let tray = tray::create(app.app_handle())?;
            let tray_handle = tray.clone();

            let app_handle = app.app_handle().clone();

            {
                let defaults: Vec<Tz> = vec![
                    "America/Chicago".parse().unwrap(),
                    "America/New_York".parse().unwrap(),
                    "Asia/Manila".parse().unwrap(),
                ];
                *app_handle.state::<SelectedTimezones>().0.lock().unwrap() = defaults;
            }

            tauri::async_runtime::spawn(async move {
                loop {
                    let now = Utc::now();

                    let tzs: Vec<Tz> = {
                        app_handle
                            .state::<SelectedTimezones>()
                            .0
                            .lock()
                            .unwrap()
                            .clone()
                    };

                    let parts: Vec<String> = tzs
                        .iter()
                        .map(|tz: &Tz| {
                            let local_time = now.with_timezone(tz).format("%H:%M");
                            format!("{}: {}", tz.name(), local_time)
                        })
                        .collect();

                    let title = if parts.is_empty() {
                        "No timezones selected".to_string()
                    } else {
                        parts.join(" | ")
                    };

                    let _ = tray_handle.set_title(Some(&title));
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
