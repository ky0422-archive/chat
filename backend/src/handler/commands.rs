use std::{io::*, net::*, sync::*};

use crate::*;

pub fn handle_command<T>(content: T, writer: Arc<Mutex<BufWriter<TcpStream>>>, channel: mpsc::Sender<String>, client_name: String) -> Result<()>
where
    T: Into<String>,
{
    let mut writer = match writer.lock() {
        Ok(writer) => writer,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("Failed to lock writer: {}", e))),
    };

    let content = content.into().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
    let (command, args) = (content[0][1..].to_string(), content[1..].to_vec());

    match command.as_str() {
        "say" => {
            handle_error(channel.send(format!("[Anonymous] {}\n", args.join(" "))));

            println!("[{}] Message [{}] {}", local(), "Anonymous", args.join(" "));
        }
        "shrug" => send_message(client_name.trim(), format!("{} {}", args.join(" "), "¯\\_(ツ)_/¯").as_str(), channel),
        _ => {
            writer.write(format!("Command not found: {}\n", command).as_bytes())?;
            writer.flush()?;
        }
    }

    Ok(())
}
