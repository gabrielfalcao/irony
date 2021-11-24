use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use ansi_term::Colour;
use std::fs;

struct File {
    path: String,
    meta: fs::Metadata,
}
fn list_files(path: &str) -> Vec<File> {
    let (tx, rx) = mpsc::channel();
    let paths = fs::read_dir(path).unwrap();
    for p in paths {
        let tx = tx.clone();
        thread::spawn(move || {
            let path = String::from(p);
            let meta = fs::metadata(path).unwrap();
            let val = File {path, meta};
            tx.send(val).unwrap();
        });
    }
    let files = Vec::new();
    loop {
        let received = rx.recv_timeout(Duration::from_millis(400)){
            Ok(val) => files.push(val),
            Err(_) => return files
        };
        println!("{}{}", Colour::Green.bold().paint(received));
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut threads = Vec::new();

    for i in 1..100 {
        let tx = tx.clone();
        threads.push(thread::spawn(move || {
            let val = format!("hi {}", i);
            tx.send(val).unwrap();
        }));
    }
    loop {
        let received = match rx.recv_timeout(Duration::from_millis(100)){
            Ok(val) => val,
            Err(_) => break
        };
        println!("{}{}", Colour::Green.bold().paint("Got: "), Colour::Blue.bold().paint(received));
    }

}
