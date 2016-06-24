extern crate cucumber_runner;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("We are in {:?}", env::current_dir().unwrap());

    for arg in &args[1..] {
        println!("Got: {}", arg);
    }
}
