use std::{
    process::Command,
    env,
    fs::{self, DirEntry, ReadDir},

};



#[derive(Debug, Clone)]
struct Arguments {
    all: bool,
    recursive: bool,
    meta: bool,
    comma: bool,
    help: bool,
    list: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Arguments {
            all: false,
            recursive: false,
            meta: false,
            comma: false,
            help: false,
            list: false,
        }
    }
}





fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut args: Vec<String> = env::args().collect();
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
            if &arg[..1] != "-" {
                match fs::read_dir(arg) {
                    Ok(dir) => {
                        args.pop();
                    }
                    Err(..) => return Err("dir not exist".into()),
                }
            } else {
                let dir = fs::read_dir(".");
            }
        }
        None => {
            let dir = fs::read_dir(".");
        }
    }



    
//    println!("{:?}", args);
//
//    let result = std::fs::read_to_string(&args.path);
//
//    let content = match result {
//        Ok(content) => content,
//        Err(error) => {
//            return Err(error.into());
//        }
//    };
//
//    let mut line_co = 1;
//    for line in content.lines() {
//        if line.contains(&args.pattern) {
//            println!(">>>>>>>> in line {} <<<<<<<<", line_co);
//            let split: Vec<&str> = line.split(&args.pattern).peekable().collect();
//            println!("{:?}", split);
//
//            println!("{}", split.len());
//
//            for s in split {
//                print!("{}", s);
//                print!("\x1b[91m{}\x1b[0m", &args.pattern);
//            }
//            println!("");
//        }
//        line_co += 1;
//    }

    Ok(())
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
