mod handle_connection;

use std::{
    net::{SocketAddr, TcpListener},
    str::FromStr,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, RwLock,
    },
    thread,
};

use handle_connection::*;

fn main() {
    let address = SocketAddr::from_str("127.0.0.1:8080").unwrap();
    let listener = TcpListener::bind(address).unwrap();

    println!("Listening on {}", address);

    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let arc = Arc::new(RwLock::new(Vec::new()));
    let arc_clone = arc.clone();

    thread::spawn(move || loop {
        loop {
            let message = rx.recv().unwrap();

            arc_clone.write().unwrap().push(message);
        }
    });

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!("New connection: {}", stream.peer_addr().unwrap());

            let (tx, arc) = (tx.clone(), arc.clone());

            thread::spawn(move || handle_connection(stream, tx, arc));
        }
    }
}
