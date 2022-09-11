// use cursive::views::*;

use std::{io::*, net::*, thread};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream);

    thread::spawn(move || {
        let mut line = String::new();

        loop {
            reader.read_line(&mut line).unwrap();

            println!("{}", line.trim());

            line.clear();
        }
    });

    loop {
        let mut reads = String::new();
        stdin().read_line(&mut reads).unwrap();

        writer
            .write(format!("{}\n", reads.trim()).as_bytes())
            .unwrap();
        writer.flush().unwrap();
    }
}
