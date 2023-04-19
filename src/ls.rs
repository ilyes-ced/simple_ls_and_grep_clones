use std::{fs, path::PathBuf, env};


struct arguments{
    all: bool,
    recursive: bool,
    meta: bool,
    comma: bool,
    time: bool,
}



fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    print!("\x1b[34m{:?}\x1b[0m", args);


    //here we proccess the arguments
    print!("\x1b[34m{:?}\x1b[0m", args.last());



    match args.last(){
        Some(arg) => {
            if &arg[..1] != "-" {
                return Err("here we make the dir the input or we throw error".into())
            }
            for arg in args{
                println!("{}", arg);
            }
        },
        None => {
            return Err("no arguments here".into())
        },
    }









    let path = fs::read_dir(".");

    let paths = match path {
        Ok(content) => {
            content
        }
        Err(error) => {
            return Err(error.into());
        }
    };

    let mut sorted_paths: Vec<_> = paths
        .map(|r| r.unwrap())
        .collect();
    
    sorted_paths.sort_by_key(|dir| dir.path());


    for path in sorted_paths {
        is_dir_or_file(path.path());
    };


    println!("");

    Ok(())
}


fn is_dir_or_file(file_dir: PathBuf) {
    let is_dir = file_dir.is_dir();
    let path = &file_dir.to_string_lossy()[2..];

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

