use chrono::prelude::*;
use std::{io::*, net::*, sync::*, thread};

use super::*;

pub fn handle_connection(stream: TcpStream, channel: mpsc::Sender<String>, arc: Arc<RwLock<Vec<String>>>) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let writer = Arc::new(Mutex::new(BufWriter::new(stream)));
    let writer_clone = writer.clone();

    let mut client_name = String::new();
    reader.read_line(&mut client_name)?;

    if let Err(e) = channel.send(format!("Welcome, {}!\n", client_name.trim())) {
        eprintln!("Error: {}", e);
    }

    thread::spawn(move || loop {
        let mut reads = String::new();

        let local = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        match reader.read_line(&mut reads) {
            Ok(size) => {
                if size == 0 {
                    if let Err(e) = channel.send(format!("{} disconnected.\n", client_name.trim())) {
                        eprintln!("Error: {}", e);
                    }

                    break;
                }

                if reads.trim().starts_with("/") {
                    if let Err(e) = handle_command(reads.trim(), writer.clone()) {
                        eprintln!("Error: {}", e);
                    };
                } else {
                    if reads.trim().len() != 0 {
                        if let Err(e) = channel.send(format!("[{}] [{}] {}\n", local, client_name.trim(), reads.trim())) {
                            eprintln!("Error: {}", e);
                        }
                    }
                }

                println!("[{local}] Message [{}] {}", client_name.trim(), reads.trim());
            }
            Err(e) => {
                eprintln!("Error: {}", e);

                break;
            }
        }
    });

    let mut position = 0;

    loop {
        match arc.read() {
            Ok(lines) => {
                match writer_clone.lock() {
                    Ok(mut writer) => {
                        for i in position..lines.len() {
                            writer.write_fmt(format_args!("{}", lines[i]))?;
                            position = lines.len();
                        }

                        writer.flush()?;
                    }
                    Err(e) => eprintln!("Error: {}", e),
                };

                thread::sleep(std::time::Duration::from_millis(500));
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
