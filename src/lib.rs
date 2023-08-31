use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    //tests
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    //代码中的字符串字面量都是该类型，且拥有 'static 生命周期
    pub fn build(args: &[String]) -> Result<Config,&'static str> {
        if args.len()<3{
            return Err("no enough arguments");
        }
        let query = &args[1].clone();
        let file_path = &args[2].clone();
        let ingore_case = env::var("IGNORE_CASE").map_or(false, |var| var.eq("1"));
        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case:ingore_case,
        })
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case{
        //如果加了;，则返回的是元组，是错误的
        search_case_insensitive(&config.query, &content)
    }else{
        search(&config.query, &content)
    };

    for line in results{
        println!("{line}");
    }
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

pub fn search_case_insensitive<'a>(query:&str,contents: &'a str)->Vec<&'a str>{
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for lines in contents.lines(){
        if lines.to_lowercase().contains(&query){
            result.push(lines);
        }
    }

    result
}

#[cfg(test)]
mod test{
    use std::vec;

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
    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."],search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}