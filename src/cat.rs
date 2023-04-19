use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    
    


        let file = File::open("src/text.txt")?;
        let reader = BufReader::new(file);
        
        for line in reader.lines() {
            println!("{}", line?);
        }


    Ok(())
}