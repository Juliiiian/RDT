// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    io::Read,
    process::{Command, Stdio},
};

// #[derive(Clone, serde::Serialize)]
// struct Payload {
//   message: String,
// }

#[tauri::command]
fn start_rustserver() -> String {
    let mut rustserver = Command::new("C:/Coding/PlayRust/rustserver_2024/RustDedicated.exe")
        .current_dir("C:/Coding/PlayRust/rustserver_2024/")
        .args([
            "-batchmode",
            "+server.port",
            "28015",
            "+server.level",
            "Procedural Map",
            "+server.seed",
            "1234",
            "+server.worldsize",
            "4000",
            "+server.maxplayers",
            "100",
            "+server.hostname",
            "Your Server Name",
            "+server.description",
            "Description Here",
            "+server.headerimage",
            "http://yourwebsite.com/serverimage.jpg",
            "+server.identity",
            "youridentity",
            "+rcon.port",
            "28016",
            "+rcon.password",
            "yourrconpassword",
            "-logfile",
            "output.txt",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    //.expect("Failed to echo");

    let mut stdout = rustserver.stdout.take().unwrap();

    let mut buf = String::new();

    while let Ok(n) = stdout.read_to_string(&mut buf) {
        print!("n: {}, {}", n, buf.as_str());
        buf.clear();
    }
    // window
    //     .get_window("main")
    //     .emit_all(
    //         "data",
    //         Payload {
    //             message: "Tauri is awesome!".into(),
    //         },
    //     )
    //     .unwrap();

    return format!("Hello, test! You've been greeted from Rust!");
}

fn main() {
    // start_rustserver();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_rustserver])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
