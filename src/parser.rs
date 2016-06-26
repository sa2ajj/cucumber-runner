use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
};

use regex::Regex;

use super::Error;
use super::Result;

use tokenizer::*;
use types::*;

#[derive(Debug)]
pub enum State {
    Init,
    TagsForFeature(Vec<String>),
    Feature,
    FeatureWithBackground,
    FeatureWithDescription,
    FeatureWithLanguage,
}

pub fn parse(filename: &str) -> Result<Feature> {
    let file = try!(File::open(filename));

    let mut feature = Feature {
        filename: filename.to_owned(),
        name: None,
        description: None,
        background: None,
        items: Vec::new(),
    };
    let mut lineno: usize = 0;      // I wish enumerate method took a start value parameter
    let mut state = State::Init;

    for line in BufReader::new(file).lines() {
        let line = try!(line);
        lineno += 1;

        state = match state {
            State::Init => {
                lazy_static! {
                    static ref RE: Regex = prepare_regex(&[FEATURE_RE, LANGUAGE_RE, TAGS_RE], true);
                }

                match try!(parse_line(&line, &RE)) {
                    Line::EmptyLine(_, _) =>
                        State::Init,

                    Line::Feature(_, _, name) => {
                        feature.name = name;

                        State::Feature
                    }

                    Line::Language(_, _, language) => {
                        println!("Language directive is ignored (lang={:?})", language);

                        State::FeatureWithLanguage
                    }

                    Line::Tags(_, _, tags) => {
                        State::TagsForFeature(tags)
                    }

                    line @ _ => {
                        return Err(Error::parse_error(filename, lineno,
                                                      &format!("unexpected line: {:?}", line)));
                    }
                }
            }

            _ => {
                return Err(Error::parse_error(filename, lineno,
                                              &format!("unhandled state: {:?}", state)));
            }
        }
    }

    Ok(feature)
}
