// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use serde_json::Number;
use tauri::{SystemTray, SystemTrayEvent};
use tauri::{CustomMenuItem, SystemTrayMenu};
extern crate osascript;
 
#[derive(Serialize)]
struct AlertParams {
    title: String,
    message: String,
    alert_type: String,
    buttons: Vec<String>,
}
 
#[derive(Deserialize)]
struct Result {
    result: Number
}
 

fn main() {

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
    .add_item(quit);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    // app.tray_handle().set_icon(tauri::Icon::Raw(include_bytes!("../path/to/myicon.ico").to_vec())).unwrap();

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
              position: _,
              size: _,
              ..
            } => {
                /*
                tell application "System Preferences"
    reveal anchor "trackpadTab" of pane id "com.apple.preference.trackpad"
  end tell
  tell application "System Events" to tell process "System Preferences"
    click checkbox 1 of tab group 1 of window 0
  end tell
  quit application "System Preferences"
  return 1 */
              println!("system tray received a left click");
              let script = osascript::JavaScript::new("
              
                const systemSettings = Application('System Settings');
                systemSettings.activate();
                systemSettings.panes.byName('Trackpad').reveal();

                delay(0.1);
                const systemEvents = Application('System Events');
                systemEvents.includeStandardAdditions = true;

                // Get the process of System Preferences
                const processSettings = systemEvents.processes['System Settings'];

                processSettings.windows.at(0).groups.at(0).splitterGroups.at(0).groups.at(1).groups.at(0).tabGroups.at(0).radioButtons.at(1).click();

                processSettings.windows.at(0).groups.at(0).splitterGroups.at(0).groups.at(1).groups.at(0).scrollAreas.at(0).groups.at(0).checkboxes.at(0).click();
                systemSettings.quit();

            ");
 
                let res: Option<Result> = script.execute().ok();
            }
            SystemTrayEvent::RightClick {
              position: _,
              size: _,
              ..
            } => {
              println!("system tray received a right click");
            }
            SystemTrayEvent::DoubleClick {
              position: _,
              size: _,
              ..
            } => {
              println!("system tray received a double click");
            }
            SystemTrayEvent::MenuItemClick { id, .. } => {
              match id.as_str() {
                "quit" => {
                  std::process::exit(0);
                }
                _ => {}
              }
            }
            _ => {}
          })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
