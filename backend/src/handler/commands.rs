use std::{io::*, net::*, sync::*};

pub fn handle_command<T>(content: T, writer: Arc<Mutex<BufWriter<TcpStream>>>) -> Result<()>
where
    T: Into<String>,
{
    let mut writer = match writer.lock() {
        Ok(writer) => writer,
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("Failed to lock writer: {}", e))),
    };

    let content = content.into().to_string();
    let _args = content.split_whitespace().collect::<Vec<&str>>()[1..].to_vec();

    match content.as_str() {
        // TODO
        _ => {
            writer.write(format!("Command not found: {}\n", content.trim()).as_bytes())?;
            writer.flush()?;
        }
    }

    Ok(())
}
