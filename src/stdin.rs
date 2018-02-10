use std::io::{self, BufRead};
use std::thread;

use futures::sync::mpsc;
use futures::{Future, Sink, Stream};

pub fn stdin() -> Box<Stream<Item=String, Error=io::Error> + Send + 'static> {
    let (mut tx, rx) = mpsc::channel(1);
    thread::spawn(move || {
        let input = io::stdin();
        for line in input.lock().lines() {
            match tx.send(line).wait() {
                Ok(s) => tx = s,
                Err(_) => break,
            }
        }
    });
    Box::new(rx.then(|e| e.unwrap()))
}
