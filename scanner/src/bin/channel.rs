use std::sync::mpsc;
fn main() {
    let (tx, rx) = mpsc::channel();

    // Send some messages
    tx.send(1).unwrap();
    tx.send(2).unwrap();

    // Drop the sender
    drop(tx.clone());
    tx.send(3).unwrap();
    tx.send(4).unwrap();

    // Receive messages until the channel is empty
    while let Ok(message) = rx.recv() {
        println!("Received: {}", message);
    }
}
