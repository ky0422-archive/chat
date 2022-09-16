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

pub fn local() -> String {
    Local::now().format("%Y-%m-%d %H:%M").to_string()
}

pub fn send_message<T>(name: T, message: T, channel: mpsc::Sender<String>)
where
    T: Into<String> + Copy,
{
    handle_error(channel.send(format!("[{}] [{}] {}\n", local(), name.into(), message.into())));
    println!("[{}] Message [{}] {}", local(), name.into(), message.into());
}
