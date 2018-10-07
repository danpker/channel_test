use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::{thread, time};

struct Message {
    id: u64,
}

fn main() {
    // You don't need to give the data type if Rust can work it out
    // let (sender, receiver): (Sender<Message>, Receiver<Message>) = mpsc::channel();
    let (sender, receiver) = mpsc::channel();

    // Need this pattern for the fn runOnce trait.
    thread::spawn(move || producer(sender));

    // Run the consumer on the main tread so it doesn't exit early.
    consumer(receiver);
}

/// Continuously send messages, forever.
fn producer(sender: Sender<Message>) {
    let mut counter: u64 = 0;
    loop {
        let message = Message{id: counter};
        sender.send(message).unwrap();
        counter = counter + 1;
    }
}

/// Continuously receive messages. If none are available, sleep to save CPU
fn consumer(receiver: Receiver<Message>) {
    loop {
        match receiver.try_iter().next() {
            Some(item) => println!("{}", item.id),
            // Sleep for 10ms if there's nothing to save CPU
            None => thread::sleep(time::Duration::from_millis(10)),
        }
    }
}
