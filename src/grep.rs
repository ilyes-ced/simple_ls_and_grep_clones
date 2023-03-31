use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("{:?}", args);

    let result = std::fs::read_to_string(&args.path);
    
    let content = match result {
        Ok(content) => {
            content
        }
        Err(error) => {
            return Err(error.into());
        }
    };

    let mut line_co = 1;
    for line in content.lines() {
        if line.contains(&args.pattern) {

            println!(">>>>>>>> in line {} <<<<<<<<", line_co);
            let split: Vec<&str> = line.split(&args.pattern).peekable().collect();
            println!("{:?}", split);

            println!("{}", split.len());

            for s in split {
                print!("{}", s);
                print!("\x1b[34m{}\x1b[0m", &args.pattern);
            }
            println!("");
        }
        line_co += 1;
    }



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