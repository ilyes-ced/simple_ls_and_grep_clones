use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::env;


fn main() -> io::Result<()> {
    let file = File::open("src/text.txt")?;
    let reader = BufReader::new(file);
    //for line in reader.lines() {
    //    println!("{}", line?);
    //}

    let mut args: Vec<String> = env::args().collect();
    
    let index = args.iter().position(|x| *x == 0).unwrap();
    args.remove(index);
    

     //let parts = "some string 123 content".split(" ");
    //let collection = parts.collect::<Vec<&str>>();
    
    print_type_of(&args);

    for arg in args {
        println!("{}", arg);
    }

    Ok(())
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
