extern crate cucumber_runner;

use std::env;

fn main() {
    let mut cwd = env::current_dir().unwrap();

    let root = cwd.clone();

    cwd.push("features");

//  let features = cucumber_runner::find_features(cwd.as_path()).unwrap();
//
//  for feature in features {
//      println!("Found: {:?}", feature);
//  }
    cucumber_runner::parse_files(root.as_path(), cwd.as_path());
}
