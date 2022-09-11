use std::{error::Error, io::*, net::*, result::Result, sync::*, thread};

pub fn handle_connection(
    stream: TcpStream,
    channel: mpsc::Sender<String>,
    arc: Arc<RwLock<Vec<String>>>,
) -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);

    writer.write(b"Enter your name\n")?;
    writer.flush()?;

    let mut client_name = String::new();
    reader.read_line(&mut client_name)?;

    if let Err(e) = channel.send(format!("Welcome, {}!\n", client_name.trim())) {
        eprintln!("Error: {}", e);
    }

    thread::spawn(move || loop {
        let mut reads = String::new();

        reader.read_line(&mut reads).unwrap();

        if reads.trim().len() != 0 {
            if let Err(e) = channel.send(format!("[{}] {}\n", client_name.trim(), reads.trim())) {
                eprintln!("Error: {}", e);
            }
        }
    });

    let mut position = 0;

    loop {
        let lines = arc.read().unwrap();

        for i in position..lines.len() {
            writer.write_fmt(format_args!("{}", lines[i]))?;
            position = lines.len();
        }

        writer.flush()?;

        thread::sleep(std::time::Duration::from_millis(100));
    }
}
