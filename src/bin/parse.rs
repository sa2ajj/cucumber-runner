extern crate cucumber_runner;

use std::env;

fn main() {
    println!("We are in {:?}", env::current_dir().unwrap());

    for arg in env::args() {
        println!("Got: {}", arg);
    }
}
