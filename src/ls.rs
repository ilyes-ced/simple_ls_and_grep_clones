use std::{fs::{self, ReadDir, DirEntry}, path::PathBuf, env};
use std::process::Command;



#[derive(Debug, Clone)]
struct Arguments{
    all: bool,
    recursive: bool,
    meta: bool,
    comma: bool,
    time: bool,
    help: bool,

}

impl Default for Arguments {
    fn default() -> Self {
        Arguments{
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


    // display help page and close programm
    match args.last(){
        Some(help) => {
            if help == "--help" {

                Command::new("cat")
                .args(["src/ls_help.txt"])
                .spawn()
                .expect("spawn failure");
            
               std::process::exit(0);
            }
        },
        _ => {}

    };

    match args.last(){
        Some(arg) => {
            if &arg[..1] != "-" {
                match fs::read_dir(arg){
                    Ok(dir) => {
                        args.pop();
                        print_dirs(dir, flags(args));
                    },
                    Err(err) => {
                        return Err("dir not exist".into())
                    }
                }
            }else{
                let dir = fs::read_dir(".");
                print_dirs(dir.unwrap(), flags(args));
            }
        },
        None => {
            let dir = fs::read_dir(".");
            print_dirs(dir.unwrap(), Arguments::default());
        },
    }

    Ok(())

}
























//recursive 
fn print_dirs(dir: ReadDir, args: Arguments) -> Vec<DirEntry> {
    let paths: ReadDir = dir;

    let mut sorted_paths: Vec<DirEntry> = paths
        .map(|r| r.unwrap())
        .collect();
    
    sorted_paths.sort_by_key(|dir| dir.path());


    //first print all
    println!("\x1b[34m>>{:?}:\x1b[0m", sorted_paths);

    for path in &sorted_paths {

        let file_dir = path.path();
        let path = file_dir.file_name().unwrap().to_str().unwrap();
        if file_dir.is_dir() {
            if &path[..1] != "." || args.all  {
                print!("\x1b[34m{}\x1b[0m", path);
                print!("    ");
            }
        }else{
            if &path[..1] != "." || args.all {
                print!("{}", path);
                print!("    ");
            }
        }
    };
    println!("");
    println!("");
    //second print all inside of dirs in recursive whe the recursive flag is given
    if args.recursive {
        for path in &sorted_paths {
            let dir = path.path();
            if dir.is_dir() {
                println!("\x1b[34m>>{}:\x1b[0m", dir.to_str().unwrap());
                print_dirs(
                    fs::read_dir(dir).unwrap(), args.clone()
                );
                println!("");
            }
        };
    }

    sorted_paths
}









fn print_all(){

}









fn flags(args: Vec<String>) -> Arguments{
    
    let mut arguments_provided = Arguments{
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