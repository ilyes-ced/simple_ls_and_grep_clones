use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::env;
use std::path::PathBuf;




fn main() -> io::Result<()> {
    let file = File::open("src/text.txt")?;
    let reader = BufReader::new(file);
    let args: Vec<String> = env::args().collect();
    
    


    for i in 1..args.len() {



        let para = args[i].split("").collect::<Vec<&str>>();
        println!("{:?}///", args[i]);
        //for j in 1..para.len() {
        //    println!("{:?}", para[j]);
        //}

        if para[1] == "/" && para[2] == "/" {
            println!(" /help ");
        }else if para[1] == "/" {
            println!(" /h ");
        }else {
            println!("other command");
        }

        let conc = "args[1].".to_owned().spush_str("borrowed_string");
        println!("{}", conc);

        let file = File::open(&conc)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            println!("{}", line?);
        }
       
        //println!("{}", arg);
        //println!("{:?}", para);
    }

    //let path = get_current_working_dir();
    //let path2 = get_current_working_dir();
    //println!("//////////////////////////////////////////////////////////");
    //print_type_of(&get_current_working_dir().unwrap().to_str());
    //print_type_of(&get_current_working_dir());
    //println!("{:?}", path);
    //println!("{:?}", path2);
    //println!("//////////////////////////////////////////////////////////");


    
    let file = File::open("src/text.txt")?;
    let reader = BufReader::new(file);
    //for line in reader.lines() {
    //    println!("{}", line?);
    //}
    Ok(())
}

fn get_current_working_dir() -> std::io::Result<PathBuf> {
    env::current_dir()
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}