extern crate cucumber_runner;

use std::env;

fn main() {
    let mut cwd = env::current_dir().unwrap();

    println!("We are in {:?}", cwd);

    cwd.push("features");

    let features = cucumber_runner::find_features(cwd.as_path()).unwrap();

    println!("Found: {:?}", features);
}
