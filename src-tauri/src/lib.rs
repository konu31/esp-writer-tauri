#[tauri::command]
fn get_board_info(port: &str) -> String {
    print!("port:{}", port);
    format!("Port:{},Info:{}", port, "ESP32")
}

#[tauri::command]
fn write_firmware(port: &str, baudrate: &str) -> String {
    print!("port:{}", port);
    print!("baudrate:{}", baudrate);
    format!("Port:{},Baudrate:{},Result:{}", port, baudrate, "Success")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_board_info,
            write_firmware
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
