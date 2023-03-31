use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {



    let path = fs::read_dir(".");

    let paths = match path {
        Ok(content) => {
            content
        }
        Err(error) => {
            return Err(error.into());
        }
    };

    for path in paths {
      
        match path {
            Ok(content) => {
                is_dir_or_file(content.path());

                content
            }
            Err(error) => {
                return Err(error.into());
            }
        };


        //match path {
        //    Ok(content) => {
        //        content
        //    }
        //    Err(error) => {
        //        return Err(error.into());
        //    }
        //};


    }

    Ok(())
}


fn is_dir_or_file(file_dir: PathBuf) {
    let is_dir = file_dir.is_dir();

    if is_dir {
        println!("\x1b[34m{}\x1b[0m", file_dir.display());
    }else{
        get_file_type(file_dir);
    }
}


fn get_file_type(file_dir: PathBuf) {
    println!("\x1b[32m{}\x1b[0m", file_dir.display());
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}