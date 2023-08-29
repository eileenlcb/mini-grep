use std::error::Error;
use std::fs;

pub struct Config {
    //tests
    pub query: String,
    pub file_path: String,
}

impl Config {
    //代码中的字符串字面量都是该类型，且拥有 'static 生命周期
    pub fn build(args: &[String]) -> Result<Config,&'static str> {
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

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    println!("{}", content);
    Ok(())
}

pub fn search<'a>(query:&str,contents: &'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    for lines in contents.lines(){
        if lines.contains(query){
            results.push(lines);
        }
    }
    results
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."],search(query,contents));
    }
}