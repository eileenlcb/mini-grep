use std::env;
use std::fs;

fn main(){
    //args不接受非unicode，如果需要，需要使用args_os
    let args:Vec<String> = env::args().collect();
    dbg!(args);

    let file_path = "poem.txt";
    let content = fs::read_to_string(file_path).expect("no such file");
    println!("{}",content);
}
