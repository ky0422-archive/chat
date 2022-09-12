mod handle_connection;

use handle_connection::*;
use std::{net::*, str::*, sync::*, thread};

fn main() {
    let address = SocketAddr::from_str("127.0.0.1:8080").unwrap();
    let listener = TcpListener::bind(address).unwrap();

    println!("Listening on {}", address);

    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    let arc = Arc::new(RwLock::new(Vec::new()));
    let arc_clone = arc.clone();

    thread::spawn(move || loop {
        let message = rx.recv().unwrap();

        arc_clone.write().unwrap().push(message);
    });

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!("New connection: {}", stream.peer_addr().unwrap());

            let (tx, arc) = (tx.clone(), arc.clone());

            thread::spawn(move || {
                if let Err(e) = handle_connection(stream, tx, arc) {
                    eprintln!("Error: {}", e);
                }
            });
        }
    }
}
