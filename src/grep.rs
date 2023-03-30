use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("{:?}", args);
    println!("{:?}", &args.path);

    let result = std::fs::read_to_string(&args.path);
    
    let content = match result {
        Ok(content) => {
            println!("File content: {:?}", content);
            content
        }
        Err(error) => {
            println!("Oh noes: {:?}", error);
            return Err(error.into());
        }
    };
    
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())

}
