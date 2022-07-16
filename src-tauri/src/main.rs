#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri::Manager;

#[derive(Debug)]
struct CalcState {
    num1: i64,
    num2: i64,
    operator: String,
}

struct CalcStateManager {
    state: Mutex<CalcState>,
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        greet,
    ])
    .setup(|app| {
        let calc_state_manager 
            = CalcStateManager{ state: Mutex::new(CalcState{ num1: 0, 
                                                             num2: 0, 
                                                             operator: String::from("")})};
        app.manage(calc_state_manager);

        Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
