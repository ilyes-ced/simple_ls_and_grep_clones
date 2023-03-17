use std::io;
use std::io::prelude::*;

fn main(){


    //let stdin = io::stdin();
//
    //
    //for line in stdin.lock().lines() {
    //    println!("{}", line.unwrap());
    //}
    


    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let buffer = stdin.fill_buf().unwrap();

    // work with buffer
    println!("{buffer:?}");
    println!("{:?}", buffer.lines());

    // ensure the bytes we worked with aren't returned again later
    let length = buffer.len();
    stdin.consume(length);
}



