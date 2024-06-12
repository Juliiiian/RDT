// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use portable_pty::{CommandBuilder, PtySize};
use std::io::{Read, Write};
use std::str;
use std::sync::{Arc, Mutex};
use tauri::{Runtime, Window};

// #[derive(Clone, serde::Serialize)]
// struct Payload {
//     message: String,
// }

async fn listen_terminal(
    reader: Arc<Mutex<Box<dyn Read + Send>>>,
    // writer: Arc<Mutex<Box<dyn Write + Send>>>,
    // t_win: Window,
) {
    let mut buffer: Vec<u8> = vec![0u8; 1];
    loop {
        let mut pty_reader = reader.lock().expect("msg");
        match pty_reader.read_exact(&mut buffer) {
            Ok(_) => {
                // Process the incoming data
                match str::from_utf8(&buffer) {
                    Ok(text) => {
                        println!("Text: {}", text);
                        // t_win
                        //     .emit(
                        //         "data",
                        //         Payload {
                        //             message: format!("[Size: {text}][Message]: {text}").into(),
                        //         },
                        //     )
                        //     .unwrap();
                        //TODO: Do something with the input, probably i would add a inter-process queue and sand the data there to be read
                    }
                    Err(e) => println!("Error during terminal reading: {}", e),
                };
            }
            Err(_) => {
                //problem with the shell stream probably it have been closed
            }
        };
    }
}

fn new_terminal(t_win: Window) {
    let cols: u16 = 100;
    let rows: u16 = 10;

    let pty = portable_pty::native_pty_system();
    let terminal = pty.openpty(PtySize {
        rows: rows,
        cols: cols,
        pixel_width: 0,
        pixel_height: 0, // put your terminal dimension I change them later by a window resize event
    });

    if terminal.is_err() {
        panic!("Problem during terminal creation");
    }
    let terminal = terminal.unwrap();

    // Spawn a shell into the pty
    let shell = match cfg!(windows) {
        true => "powershell",
        false => "bash",
    };

    println!("{shell}");

    let cmd = CommandBuilder::new(shell);
    let child = terminal.slave.spawn_command(cmd);
    if child.is_err() {
        panic!("Error during the creation of the shell instance.");
    }
    let child = child.unwrap();

    let process_id = child.process_id().expect("process_id");

    println!("Process ID: {}", process_id);

    // Release any handles owned by the slave: we don't need it now
    // that we've spawned the child.
    drop(terminal.slave);

    println!("Started process");
    // Save these somewhere you can access them back if you need to close the streams
    let writer = Arc::new(Mutex::new(terminal.master.take_writer().unwrap()));
    let reader = Arc::new(Mutex::new(terminal.master.try_clone_reader().unwrap()));
    // let master = Arc::new(Mutex::new(terminal.master));

    //Start thread to listen to the created terminal (we need a thread for each terminal since in some os read on stdin is blocking)
    tauri::async_runtime::spawn(async move {
        // This is running in a separated thread!
        //writer.clone().lock().unwrap(),
        listen_terminal(reader).await;
    });
    // Now we can continue and write anything we want on the shell
    //writeln!(mux_writer, "ls");
    //using writeln!() will immediately execute the command like pressing enter
    let mut mux_writer = writer.lock().unwrap();

    println!("Start writing!");

    writeln!(&mut mux_writer, "cd C:/Coding/PlayRust/rustserver_2024/").expect("cant cd");

    println!("Start rustserver!");
    writeln!(&mut mux_writer, "C:/Coding/PlayRust/rustserver_2024/RustDedicated.exe -batchmode +server.port 28015 +server.level \"Procedural Map\" +server.seed 1234 +server.worldsize 4000 +server.maxplayers 100 +server.hostname \"Your Server Name\" +server.description \"Description Here\"   +server.url \"http://yourwebsite.com\" +server.headerimage \"http://yourwebsite.com/serverimage.jpg\" +server.identity \"youridentity\" +rcon.port 28016 +rcon.password \"yourrconpassword\" -logfile \"output.txt\"").expect("cant start rustdedicated");
    // return mux_writer;
}

#[tauri::command]
fn start_rustserver(t_win: Window) -> String {
    println!("Starting Terminal:");

    new_terminal(t_win);

    return format!("Hello, test! You've been greeted from Rust!");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![start_rustserver])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
