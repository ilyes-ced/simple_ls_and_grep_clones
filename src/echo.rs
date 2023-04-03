use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("{:?}", args);

    //let result = std::fs::read_to_string(&args.path);
    //
    //let content = match result {
    //    Ok(content) => {
    //        println!("{}", &content);
    //        content
    //    }
    //    Err(error) => {
    //        return Err(error.into());
    //    }
    //};





    Ok(())

}

