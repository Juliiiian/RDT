// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{BufRead, BufReader, Read, Write};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::Mutex;
use std::thread;

use tauri::Window;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn start_process(t_win: Window, sender: Sender<String>, receiver: Receiver<String>) {
    let mut child = Command::new("C:/Coding/PlayRust/rustserver_2024/RustDedicated.exe")
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
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Could not spawn.");

    println!("Started process: {}", child.id());
    let stdout = child.stdout.take().unwrap();

    thread::spawn(move || {
        let mut f = BufReader::new(stdout);

        loop {
            println!("Reading: Buf");
            let mut buf = String::new();
            match f.read_line(&mut buf) {
                Ok(result) => {
                    // sender.send(buf).unwrap();
                    // println!("[Size: {result}][Message]: {buf}");
                    t_win
                        .emit(
                            "data",
                            Payload {
                                message: format!("[Size: {result}][Message]: {buf}").into(),
                            },
                        )
                        .unwrap();
                }
                Err(e) => {
                    println!("an error!: {:?}", e);
                }
            }
        }
    });
}

#[tauri::command]
fn start_rustserver(t_win: Window) -> String {
    let (tx1, rx1) = channel();
    let (tx2, rx2) = channel();
    start_process(t_win, tx1, rx2);

    // tx2.send(String::from("Command 1\n")).unwrap();
    // start_command_thread(Mutex::new(tx2));

    // for line in rx1 {
    //     println!("Got this back: {}", line);
    // }

    return format!("Hello, test! You've been greeted from Rust!");
}

fn main() {
    // start_rustserver();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_rustserver])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
