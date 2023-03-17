use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let mut records = HashMap::new();

    while let Some(label) = lines.next() {
        let value = lines.next().expect("No value for label");
        records.insert(label.unwrap(), value.unwrap());
    }
}
