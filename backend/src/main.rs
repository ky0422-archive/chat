use backend::*;
use std::{net::*, str::*, sync::*, thread};
use threadpool::*;

fn main() {
    let address = SocketAddr::from_str("127.0.0.1:8080").unwrap();
    let listener = TcpListener::bind(address).unwrap();

    println!("Listening on {}", address);

    let (tx, rx) = mpsc::channel();
    let arc = Arc::new(RwLock::new(Vec::new()));
    let arc_clone = arc.clone();

    thread::spawn(move || loop {
        match rx.recv() {
            Ok(message) => match arc_clone.write() {
                Ok(mut lines) => lines.push(message),
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    });

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            println!(
                "New connection: {}",
                match stream.peer_addr() {
                    Ok(addr) => addr.to_string(),
                    Err(_) => "Unknown".to_string(),
                }
            );

            let pool = ThreadPool::new(4);

            let (tx, arc) = (tx.clone(), arc.clone());

            pool.execute(|| {
                handle_error(handle_connection(stream, tx, arc));
            });
        }
    }
}
