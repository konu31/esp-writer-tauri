// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Windowsでコンソールが表示される場合は以下の設定にする
// #![cfg_attr(
//     all(target_os = "windows"),
//     windows_subsystem = "windows"
// )]

fn main() {
    esp_writer_lib::run()
}
