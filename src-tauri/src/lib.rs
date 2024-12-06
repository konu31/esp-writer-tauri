use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::Command;
use tauri_plugin_shell::process::CommandEvent;
use tauri::Emitter;


fn send_to_frontend(app: &tauri::AppHandle, event_name: &str, message: String) {
    app.emit(event_name, message).unwrap();
}


#[tauri::command]
fn get_port_list(app: tauri::AppHandle) {
    let sidecar_command: Command = app
        .shell()
        .sidecar("esptool_ex")
        .unwrap()
        .args(["port_list"])
        .env("PYTHONUNBUFFERED", "1");
    let (mut rx, mut _child) = sidecar_command.spawn().unwrap();

    // 非同期で標準出力を読み取る
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    println!("Sidecar stdout: {}", line);
                    send_to_frontend(&app, "port_list", line.to_string())
                }
                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    eprintln!("Sidecar stderr: {}", line);
                }
                _ => {}
            }
        }
    });
}


#[tauri::command]
fn get_board_info(port: &str, app: tauri::AppHandle) {
    let sidecar_command: Command = app
        .shell()
        .sidecar("esptool_ex")
        .unwrap()
        .args(["board_info", port])
        .env("PYTHONUNBUFFERED", "1");
    let (mut rx, mut _child) = sidecar_command.spawn().unwrap();

    // 非同期で標準出力を読み取る
    let detect_chip_message: &str = "Detecting chip type... ";
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    println!("Sidecar stdout: {}", line);
                    let line_string: String = line.to_string();
                    send_to_frontend(&app, "board_info", line_string);

                    if line.contains(detect_chip_message) {
                        let chip: String = line.replace(detect_chip_message, "").trim().to_string();
                        println!("detect chip: {}", chip);
                        send_to_frontend(&app, "select_chip", chip);
                    }
                }
                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    eprintln!("Sidecar stderr: {}", line);
                    send_to_frontend(&app, "board_info", format!("error: {}", line.to_string()))
                }
                _ => {}
            }
        }
    });
}


#[tauri::command]
fn write_firmware(port: &str, chip: &str, baudrate: &str, app: tauri::AppHandle) {
    let sidecar_command: Command = app
        .shell()
        .sidecar("esptool_ex")
        .unwrap()
        .args(["write_firmware", port, chip, baudrate])
        .env("PYTHONUNBUFFERED", "1");
    let (mut rx, mut _child) = sidecar_command.spawn().unwrap();

    // 非同期で標準出力を読み取る
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    println!("Sidecar stdout: {}", line);
                    send_to_frontend(&app, "write_firmware", line.to_string())
                }
                CommandEvent::Stderr(line_bytes) => {
                    let line = String::from_utf8_lossy(&line_bytes);
                    eprintln!("Sidecar stderr: {}", line);
                    send_to_frontend(&app, "write_firmware", format!("error: {}", line.to_string()))
                }
                _ => {}
            }
        }
    });
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
