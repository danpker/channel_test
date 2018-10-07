use std::sync::mpsc;
use std::thread;


fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        sender.send("hello").unwrap();
    });

    println!("{}", receiver.recv().unwrap());
}
