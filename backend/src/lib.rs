pub mod handler;

use chrono::*;
use std::sync::mpsc;

pub use handler::*;

pub fn handle_error<T, E>(result: Result<T, E>)
where
    E: std::error::Error,
{
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}

pub fn send_message<T>(name: T, message: T, channel: mpsc::Sender<String>)
where
    T: Into<String> + Copy,
{
    let local = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    handle_error(channel.send(format!("[{}] [{}] {}\n", local, name.into(), message.into())));
    println!("[{}] Message [{}] {}", local, name.into(), message.into());
}
