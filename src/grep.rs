use std::{
    process::Command,
    env,
    fs::{self, DirEntry, ReadDir}, fmt::format, path,
};



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut errors: Vec<String> = vec![];

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let binding = args.clone();
    let pattren = binding.get(0).unwrap();
    args.remove(0);

    match args.last() {
        Some(help) => {
            if help == "--help" {
                Command::new("cat")
                    .args(["src/grep_help.txt"])
                    .spawn()
                    .expect("spawn failure");

                std::process::exit(0);
            }
        }
        _ => {}
    };

    match args.last() {
        Some(arg) => {
            
            for arg in &args{
                


                match fs::read_dir(arg) {
                    Ok(dir) => {
                        read_dir(dir, pattren);
                    },
                    Err(err) => {
                        match fs::read_to_string(arg) {
                            Ok(file) => {
                                read_file(file, pattren, arg);
                            }
                            Err(err) => {
                                errors.push(format!("'{}' {}", arg , err.to_string()));
                            },
                        }; 
                    }
                }




            };

            
        },
        None => {
            // raed in currrent dir
            let file = fs::read_dir(".");
        }
    }

    for error in errors{
        println!("\x1b[91m{}\x1b[0m", error);
    }
    Ok(())
}




















fn read_dir(dir : ReadDir, pattren: &String) {
    let dirs: Vec<DirEntry> = dir.map(|r| r.unwrap()).collect();
    for dir in dirs{
        match fs::read_dir(dir.path()) {
            Ok(dir) => {
                read_dir(dir, pattren);
            },
            Err(err) => {
                match fs::read_to_string(dir.path()) {
                    Ok(file) => {
                        read_file(file, pattren, dir.path().to_str().unwrap());
                    }
                    Err(err) => {
                        //errors.push(format!("'{}' {}", dir.path().to_str().unwrap() , err.to_string()));
                    },
                }; 
            }
        }
    }
}






fn read_file(file: String, pattren: &String, path: &str) {
    //println!("is file: {:?} ", file);
    let mut line_co = 1;
    for line in file.lines() {
        if line.contains(pattren) {
            print!("\x1b[94m{} in line{}:\x1b[0m" ,path , line_co);
            let mut split= line.split(pattren).peekable();
            while let Some(s) = split.next()  {
                if !split.peek().is_none() {
                    print!("{}", s);
                    print!("\x1b[91m{}\x1b[0m", pattren);
                }else{
                    print!("{}", s);
                }
            }
            
            //for s in split {
            //    print!("{}", s);
            //    print!("\x1b[91m{}\x1b[0m", pattren);
            //}
            println!("");
        }
        line_co += 1;
    }
}































/*
//////////////bg
Red 31
Green 32
Yellow 33
Blue 34
Magenta 35
Cyan 36
White 37
BrightBlack 90
BrightRed 91
BrightGreen 92
BrightYellow 93
BrightBlue 94
BrightMagenta 95
BrightCyan 96
BrightWhite 97

//////////////fg

Black 40
Red 41
Green 42
Yellow 43
Blue 44
Magenta 45
Cyan 46
White 47
BrightBlack 100
BrightRed 101
BrightGreen 102
BrightYellow 103
BrightBlue 104
BrightMagenta 105
BrightCyan 106
BrightWhite 107
*/
