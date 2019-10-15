extern crate regex;

use regex::Regex;
use std::{env, process};

pub fn parse() -> (Vec<String>, Vec<String>) {
    let args: Vec<String> = env::args().collect();
    let options           = args
                                .into_iter()
                                .filter(|arg| arg.starts_with("--"))
                                .collect();

    let args: Vec<String> = env::args().collect();
    let _args             = args
                                .into_iter()
                                .filter(|arg| !arg.starts_with("--"))
                                .collect();

    return (options, _args)
}

pub fn get_url() -> String {
    let (_options, _args) = parse();

    if _args.len() < 2 {
        println!("\nURL missing!\n");
        process::exit(0x0100);         
    }

    let url = _args[1].to_string();
    let re  = Regex::new(r"^http[s]{0,1}://").unwrap();

    if !re.is_match(&url) {
        return format!("https://{}", url);
    }

    return url; 
}
