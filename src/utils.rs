use std::io::{
    BufReader,
    BufRead,
};
use std::fs::File;
use std::path::Path;

use colored::*;
use regex::Regex;

use super::Result;

use super::parse;

use tokenizer::*;
use types::*;

pub fn find_features(root: &Path) -> Result<Features> {
    let mut result: Features = vec!();

    for entry in try!(root.read_dir()) {
        let entry = try!(entry);
        let path = entry.path();

        if try!(entry.file_type()).is_dir() {
            result.extend_from_slice(&try!(find_features(&path)));
        } else if path.extension().map_or(false, |ext| ext == "feature") {
            result.push(try!(parse(path.to_str().unwrap())));
        }
    }

    Ok(result)
}

pub fn parse_files(root: &Path, cwd: &Path) {
    for entry in cwd.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().unwrap().is_dir() {
            parse_files(root, &path);
        } else if path.extension().map_or(false, |ext| ext == "feature") {
            let friendly_name = path.strip_prefix(root).unwrap();

            parse_lines(path.to_str().unwrap(),
                        friendly_name.to_str().unwrap());
        }
    }
}

pub fn parse_lines(filename: &str, name: &str) {
    lazy_static! {
        static ref RE: Regex = prepare_regex(&[LANGUAGE_RE,
                                               TAGS_RE,
                                               FEATURE_RE,
                                               BACKGROUND_RE,
                                               SCENARIO_RE,
                                               SCENARIO_OUTLINE_RE,
                                               GIVEN_STEP_RE,
                                               WHEN_STEP_RE,
                                               THEN_STEP_RE,
                                               AND_STEP_RE,
                                               BUT_STEP_RE,
                                               EXAMPLES_RE,
                                               TABLE_RE,
                                               DOCSTRING_RE,
                                               OTHER_RE,
                                            ], true);
    }

    let file = File::open(filename).unwrap();

    let mut lineno: usize = 0;      // I wish enumerate method took a start value parameter

    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        lineno += 1;

        println!("{}:{}: {}", name, lineno, line);

        let lineno = format!("{}", lineno);

        match parse_line(&line, &RE) {
            Ok(result) => {
                let result = format!("{:?}", result);

                println!("{}:{}: {}", name.bold(), lineno.bold(), result.green());
            }

            Err(result) => {
                let result = format!("{:?}", result);

                println!("{}:{}: {}", name.bold(), lineno.bold(), result.red());
            }
        }
    }
}
