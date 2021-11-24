use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use ansi_term::Colour;
use std::fs;


#[derive(Debug)]
struct File {
    path: String,
    is_dir: bool,
    is_file: bool,
}
fn list_files(path: &str) -> Vec<File> {
    let (tx, rx) = mpsc::channel();
    let paths = fs::read_dir(path).unwrap();

    for r in paths {
        let file = r.unwrap();
        let tx = tx.clone();
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(300));
            let path = String::from(file.path().to_str().unwrap());
            let is_dir = file.metadata().unwrap().is_dir();
            let is_file = file.metadata().unwrap().is_file();
            let val = File {path: path.clone(), is_dir, is_file};
            tx.send(val).unwrap();
        });
    }
    let mut files = Vec::new();
    loop {
        match rx.recv_timeout(Duration::from_millis(400)){
            Ok(val) => {println!("{}", Colour::Green.bold().paint(format!("{:#?}", val)));files.push(val);},
            Err(_) => return files
        };

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
