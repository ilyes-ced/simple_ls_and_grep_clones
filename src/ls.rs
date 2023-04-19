use std::{
    fs::{self, ReadDir, DirEntry},
    env,
    os::{unix::prelude::{MetadataExt}}
};
use std::process::Command;



#[derive(Debug, Clone)]
struct Arguments{
    all: bool,
    recursive: bool,
    meta: bool,
    comma: bool,
    time: bool,
    help: bool,
    list: bool,
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
            list: false,
        }
    }
}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);


    // display help page and close programm if --help
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


    if args.help {
        println!("ls: invalid option -- 'z'\nTry 'ls --help' for more information.");
        std::process::exit(0);
    };

    let paths: ReadDir = dir;

    let mut sorted_paths: Vec<DirEntry> = paths
        .map(|r| r.unwrap())
        .collect();
    
    sorted_paths.sort_by_key(|dir| dir.path());


    //first print all
    for path in &sorted_paths {

        let file_dir = path.path();
        let path = file_dir.file_name().unwrap().to_str().unwrap();
        if file_dir.is_dir() {
            //print!("{:?}", file_dir);
            //print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().len());
            //print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().modified());
            //print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().uid());
            //print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().gid());
            print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().mode());
            let meta = fs::metadata(file_dir.as_os_str()).unwrap().mode();
            premessions(meta);


            if &path[..1] != "." || args.all  {
                if args.list || args.meta || args.time {
                    println!("\x1b[94m{}\x1b[0m", path);
                }else{
                    print!("\x1b[94m{}\x1b[0m", path);
                    if args.comma {
                        print!(", ");
                        println!("");
                    }else{
                        print!("    ");
                    }
                }
            }
        }else{
            //print!("{:?}", file_dir);
            //print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().len());
            //print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().modified());
            //print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().uid());
            //print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().gid());
            print!("{:?}", fs::metadata(file_dir.as_os_str()).unwrap().mode());
            let meta = fs::metadata(file_dir.as_os_str()).unwrap().mode();
            premessions(meta);


            if &path[..1] != "." || args.all {
                if args.list || args.meta || args.time {
                    println!("{}", path);
                }else{
                    print!("{}", path);
                    if args.comma {
                        print!(", ");
                        println!("");
                    }else{
                        print!("    ");
                    }
                }
            }
        }
    };
    //println!("");
    //second print all inside of dirs in recursive whe the recursive flag is given
    if args.recursive {
        for path in &sorted_paths {
            let dir = path.path();
            if dir.is_dir() {
                println!("\x1b[94m>>{}:\x1b[0m", dir.to_str().unwrap());
                print_dirs(
                    fs::read_dir(dir).unwrap(), args.clone()
                );
                println!("");
            }
        };
    }

    sorted_paths
}














fn flags(args: Vec<String>) -> Arguments{
    
    let mut arguments_provided = Arguments{
        all: false,
        recursive: false,
        meta: false,
        comma: false,
        time: false,
        help: false,
        list: false,
    };

    for arg in args{
        if &arg[..1] == "-" {
            let sub_args = arg.split("");
            let sub_args: Vec<&str> = sub_args.into_iter().filter(|&x| x != "" && x != "-").collect();

            for sub_arg in sub_args{
                match sub_arg {
                    "a" => {
                        arguments_provided.all = true;
                    },
                    "r" => {
                        arguments_provided.recursive = true;
                    },
                    "m" => {
                        arguments_provided.meta = true;
                    },
                    "c" => {
                        arguments_provided.comma = true;
                    },
                    "t" => {
                        arguments_provided.time = true;
                    },
                    "l" => {
                        arguments_provided.list = true;
                    },
                    random => {
                        println!("random /**************************  {:?}", random);
                        arguments_provided.help = true;
                    }
                };
            }

        }
    };

    arguments_provided
    
}


fn premessions( mode: u32 ){

    print!("{}",if (mode & (0x1<<9)) >= 1 {"d"}else{"-"});
    print!("{}",if (mode & (0x1<<8)) >= 1 {"r"}else{"-"});
    print!("{}",if (mode & (0x1<<7)) >= 1 {"w"}else{"-"});
    print!("{}",if (mode & (0x1<<6)) >= 1 {"x"}else{"-"});
    print!("{}",if (mode & (0x1<<5)) >= 1 {"r"}else{"-"});
    print!("{}",if (mode & (0x1<<4)) >= 1 {"w"}else{"-"});
    print!("{}",if (mode & (0x1<<3)) >= 1 {"x"}else{"-"});
    print!("{}",if (mode & (0x1<<2)) >= 1 {"r"}else{"-"});
    print!("{}",if (mode & (0x1<<1)) >= 1 {"w"}else{"-"});
    println!("{}",if (mode & 0x1) >= 1 {"x"}else{"-"});

}