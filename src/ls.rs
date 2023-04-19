use std::{fs::{self, ReadDir, File}, path::PathBuf, env, io::{BufReader, BufRead}};
use std::process::Command;



#[derive(Debug)]
struct arguments{
    all: bool,
    recursive: bool,
    meta: bool,
    comma: bool,
    time: bool,
    help: bool,

}

impl Default for arguments {
    fn default() -> Self {
        arguments{
            all: false,
            recursive: false,
            meta: false,
            comma: false,
            time: false,
            help: false,
        }
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);



    let mut arguments_provided = arguments{
        all: false,
        recursive: false,
        meta: false,
        comma: false,
        time: false,
        help: false,
    };

    // display help page and close programm
    if args.last().unwrap() == "--help" {

        Command::new("cat")
            .args(["src/ls_help.txt"])
            .spawn()
            .expect("spawn failure");

        std::process::exit(0);

    }

    match args.last(){
        Some(arg) => {
            if &arg[..1] != "-" {
                match fs::read_dir(arg){
                    Ok(dir) => {
                        args.pop();
                        println!("hello there here we have args where final is dir");
                        start(dir, flags(args));
                    },
                    Err(err) => {
                        return Err("dir not exist".into())
                    }
                }
            }else{
                let dir = fs::read_dir(".");
                println!("hello hello there here we have args");
                start(dir.unwrap(), flags(args));
            }
        },
        None => {
            let dir = fs::read_dir(".");
            println!("hello no args");
            start(dir.unwrap(), arguments::default());
        },
    }








    Ok(())
}













fn is_dir_or_file(file_dir: PathBuf) {
    let is_dir = file_dir.is_dir();
    let path = file_dir.file_name().unwrap().to_str().unwrap();

    if is_dir {
        if &path[..1] != "." {
            print!("\x1b[34m{}\x1b[0m", path);
            print!("    ");
        }
    }else{
        if &path[..1] != "." {
            print!("{}", path);
            print!("    ");
        }
    }
}



























fn start(path: ReadDir, args: arguments) {
    println!("{:?}", args);

    let paths = path;

    let mut sorted_paths: Vec<_> = paths
        .map(|r| r.unwrap())
        .collect();
    
    sorted_paths.sort_by_key(|dir| dir.path());


    for path in sorted_paths {
        is_dir_or_file(path.path());
    };


    println!("");
}


















fn print_all_sub_dirs(file_dir: PathBuf) {
    let is_dir = file_dir.is_dir();
    let path = file_dir.file_name().unwrap().to_str().unwrap();

    if is_dir {
        if &path[..1] != "." {
            print!("\x1b[34m{}\x1b[0m", path);
            print!("    ");
        }
    }else{
        if &path[..1] != "." {
            print!("{}", path);
            print!("    ");
        }
    }
}














fn flags(args: Vec<String>) -> arguments{
    
    let mut arguments_provided = arguments{
        all: false,
        recursive: false,
        meta: false,
        comma: false,
        time: false,
        help: false,
    };

    for arg in args{
        if &arg[..1] == "-" {
            let sub_args = arg.split("");
            for sub_arg in sub_args{
                match sub_arg {
                    "a" => {
                        arguments_provided.all = true;
                        "ok"
                    },
                    "r" => {
                        arguments_provided.recursive = true;
                        "ok"
                    },
                    "m" => {
                        arguments_provided.meta = true;
                        "ok"
                    },
                    "c" => {
                        arguments_provided.comma = true;
                        "ok"
                    },
                    "t" => {
                        arguments_provided.time = true;
                        "ok"
                    },
                    random => {
                        arguments_provided.help = true;
                        "try ls --help"
                    }
                };
            }

        }
    };

    arguments_provided
    
}