use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    net::TcpStream,
    sync::{mpsc::Sender, Arc, RwLock},
    thread,
};

pub fn handle_connection(
    stream: TcpStream,
    channel: Sender<String>,
    arc: Arc<RwLock<Vec<String>>>,
) {
    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream);

    writer.write(b"Name:").unwrap();
    writer.flush().unwrap();

    let mut client_name = String::new();
    reader.read_line(&mut client_name).unwrap();

    writer
        .write_fmt(format_args!("Welcome, {}!", client_name.trim()))
        .unwrap();
    writer.flush().unwrap();

    thread::spawn(move || loop {
        let mut reads = String::new();

        reader.read_line(&mut reads).unwrap();

        if reads.trim().len() != 0 {
            channel
                .send(format!("{}: {}", client_name.trim(), reads.trim()))
                .unwrap();
        }
    });

    let mut position = 0;

    loop {
        let lines = arc.read().unwrap();

        for i in position..lines.len() {
            writer.write_fmt(format_args!("{}", lines[i])).unwrap();
            writer.flush().unwrap();

            position += 1;
        }

        thread::sleep(std::time::Duration::from_millis(100));
    }
}
