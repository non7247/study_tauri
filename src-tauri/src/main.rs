#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::sync::Mutex;
use tauri::{Manager, State};

#[derive(Debug)]
struct CalcState {
    x_number: i64,
    y_number: i64,
    operator: String,
}

struct CalcStateManager {
    state: Mutex<CalcState>,
}

impl CalcStateManager {
    pub fn new() -> Self {
        CalcStateManager {
            state: Mutex::new(CalcState{ x_number: 0,
                                         y_number: 0,
                                         operator: String::from("") })
        }
    }

    pub fn clear(&self) {
        let mut calc_state = self.state.lock().unwrap();

        calc_state.x_number = 0;
        calc_state.y_number = 0;
        calc_state.operator = String::from("");
    }

    pub fn display_text(&self) -> String {
        let calc_state = self.state.lock().unwrap();

        if &calc_state.operator == "" {
            calc_state.x_number.to_string()
        } else {
            calc_state.y_number.to_string()
        }
    }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        greet,
        calc_clear,
    ])
    .setup(|app| {
        let calc_state_manager = CalcStateManager::new();
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

#[tauri::command]
fn calc_clear(calc_state_manager: State<'_, CalcStateManager>) -> String {
    calc_state_manager.clear();
    calc_state_manager.display_text()
}
