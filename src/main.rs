extern crate easy_http_request;

use easy_http_request::DefaultHttpRequest;
use std::env;
use std::{thread, time};

// XXX: remotely call bot servers and deploy this script and dependencies
// XXX: remotely call bot servers and execute this program simultaneously

fn main() {
    //let handles = (0..200)
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

fn ping_many_times(x: i32) {
    let args: Vec<String> = env::args().collect();
    let url               = &args[1];

    let response = DefaultHttpRequest::get_from_url_str(&url)
                       .unwrap()
                       .send  ()
                       .unwrap();

    print!("[{}] ",  x);
    print!("{} ",    response.status_code);
    println!("{:?}", response.headers);
    thread::sleep(time::Duration::from_millis(3));
    ping_many_times(x);
}
