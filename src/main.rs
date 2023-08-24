use std::env;
use std::fs;
use std::process;

fn main() {
    //args不接受非unicode，如果需要，需要使用args_os
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err|{
        println!("Problem is {err}");
        process::exit(1);
    });

    let content = fs::read_to_string(config.file_path).expect("no such file");
    println!("{}", content);
}

struct Config {
    //test
    query: String,
    file_path: String,
}

impl Config {
    //代码中的字符串字面量都是该类型，且拥有 'static 生命周期
    fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len()<3{
            return Err("no enough arguments");
        }
        let query = &args[1].clone();
        let file_path = &args[2].clone();
        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
        })
    }
}

// fn parse_config(args:&Vec<String>) -> Config{
//     let query = &args[1].clone();
//     let file_path = &args[2].clone();
//     Config { query: query.to_string(), file_path: file_path.to_string() }
// }
