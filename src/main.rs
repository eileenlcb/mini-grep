use std::env;
use std::process;

use minigrep::Config;

fn main() {
    //args不接受非unicode，如果需要，需要使用args_os
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        eprintln!("Problem is {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application err:{e}");
        process::exit(1);
    };

    
}