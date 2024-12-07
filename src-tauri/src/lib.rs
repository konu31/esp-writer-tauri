use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::Command;
use tauri_plugin_shell::process::CommandEvent;
use tauri::Emitter;
use tauri::async_runtime::Receiver;


// フロントエンドにメッセージを送信
fn send_to_frontend(app: &tauri::AppHandle, event_name: &str, message: String) {
    app.emit(event_name, message).unwrap();
}

// sidecarで実行したプロセスの標準出力をフロントエンドに送信
fn read_output_async(mut rx: Receiver<CommandEvent>, app: tauri::AppHandle, event_name: String) {
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    println!("Sidecar stdout: {}", line);
                    let line_string: String = line.to_string();
                    send_to_frontend(&app, &event_name, line_string);
                }
                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    eprintln!("Sidecar stderr: {}", line);
                    send_to_frontend(&app, &event_name, format!("error: {}", line.to_string()))
                }
                _ => {}
            }
        }
    });
}


#[tauri::command]
fn get_port_list(app: tauri::AppHandle) {
    let sidecar_command: Command = app
        .shell()
        .sidecar("esptool_ex")
        .unwrap()
        .args(["port_list"])
        .env("PYTHONUNBUFFERED", "1");
    let (rx, mut _child) = sidecar_command.spawn().unwrap();
    read_output_async(rx, app, "port_list".to_string())
}


#[tauri::command]
fn get_board_info(port: &str, app: tauri::AppHandle) {
    let sidecar_command: Command = app
        .shell()
        .sidecar("esptool_ex")
        .unwrap()
        .args(["board_info", port])
        .env("PYTHONUNBUFFERED", "1");
    let (rx, mut _child) = sidecar_command.spawn().unwrap();
    read_output_async(rx, app, "board_info".to_string())
}


#[tauri::command]
fn write_firmware(port: &str, chip: &str, baudrate: &str, app: tauri::AppHandle) {
    let sidecar_command: Command = app
        .shell()
        .sidecar("esptool_ex")
        .unwrap()
        .args(["write_firmware", port, chip, baudrate])
        .env("PYTHONUNBUFFERED", "1");
    let (rx, mut _child) = sidecar_command.spawn().unwrap();
    read_output_async(rx, app, "write_firmware".to_string())
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        // sidecar
        .invoke_handler(tauri::generate_handler![
            get_port_list,
            get_board_info,
            write_firmware
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
