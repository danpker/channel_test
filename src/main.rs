use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

struct Message {
    id: u64,
}

fn main() {
    let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();

    thread::spawn(move || producer(sender));
    consumer(receiver);
}

fn producer(sender: Sender<Message>) {
    let mut counter: u64 = 0;
    loop {
        let message = Message{id: counter};
        sender.send(message).unwrap();
        counter = counter + 1;
    }
}

fn consumer(receiver: Receiver<Message>) {
    loop {
        match receiver.try_iter().next() {
            Some(item) => println!("{}", item.id),
            // Sleep for 10ms if there's nothing to save CPU
            None => thread::sleep(time::Duration::from_millis(10)),
        }
    }
}
