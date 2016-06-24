use std::fs::File;
use std::io::{
    BufReader,
    BufRead,
};

use regex::{
    Regex,
    RegexBuilder,
};

use super::Error;
use super::Result;

use types::*;

pub enum State {
    Init,
    FeatureAndTags,
    Feature,
    FeatureWithBackground,
    FeatureWithDescription,
    FeatureWithLanguage,
    FeatureWithTags,
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
    let mut state = State::Init;

    for line in BufReader::new(file).lines() {
        let line = try!(line);

        match state {
            State::Init => {
                lazy_static! {
                    static ref RE: Regex = prepare_regex(&[FEATURE_RE, LANGUAGE_RE, TAGS_RE], true);
                }

                match try!(parse_line(&line, &RE)) {
                    _ =>
                        unimplemented!()
                }
            }

            _ => {
                return Err(Error::from_str("unhandled state"));
            }
        }
    }

    Ok(feature)
}

// nay be used only once and we ignore it
static LANGUAGE_RE: &'static str = r"(Language:\s*(?P<language>\S+))";

static TAGS_RE: &'static str = r"(?P<tags>@\w+(\s+@\w+)*)";

static FEATURE_RE: &'static str = r"(Feature:\s*(?P<feature>.*))";
static BACKGROUND_RE: &'static str = r"(Background:\s*(?P<background>.*))";
static SCENARIO_RE: &'static str = r"(Scenario:\s*(?P<scenario>.*))";
static SCENARIO_OUTLINE_RE: &'static str = r"(Scenario Outline:\s*(?P<outline>.*))";
static GIVEN_STEP_RE: &'static str = r"(Given\s+(?P<given>.*))";
static WHEN_STEP_RE: &'static str = r"(When\s+(?P<when>.*))";
static THEN_STEP_RE: &'static str = r"(Then\s+(?P<then>.*))";
static AND_STEP_RE: &'static str = r"(And\s+(?P<and>.*))";
static BUT_STEP_RE: &'static str = r"(But\s+(?P<but>.*))";
static EXAMPLES_RE: &'static str = r"(Examples:\s*(?P<examples>.*))";

static TABLE_RE: &'static str = r"(?P<table>\|.*\|)";
static DOCSTRING_RE: &'static str = "(?P<docstring>\"\"\"|''')";

static OTHER_RE: &'static str = "(?P<other>[^#]*)";

static INDENT_RE: &'static str = r"(?P<indent>\s*)";
static COMMENT_RE: &'static str = r"\s*(#\s*(?P<comment>.*))?";

fn prepare_regex(bits: &[&str], detect_comment: bool) -> Regex {
    let actual_bits = &bits.join("|");
    let mut pattern: Vec<&str> = vec!("^",
                                      INDENT_RE,
                                      "(",
                                      &actual_bits,
                                      ")");
    if detect_comment {
        pattern.push(COMMENT_RE);
    }

    pattern.push("$");

    RegexBuilder::new(&pattern.join(r""))
                 .case_insensitive(true)
                 .ignore_whitespace(true)
                 .compile()
                 .unwrap()
}

enum LineKind {
    EmptyLine,
    Tags,
    Feature,
    Background,
    Scenario,
    ScenarioOutline,
    GivenStep,
    WhenStep,
    ThenStep,
    AndStep,
    ButStep,
    Examples,
    Table,
    DocString,
    Other,
}

struct Line {
    pub kind: LineKind,
    pub value: Option<String>,
    pub indent: usize,
    pub comment: Option<String>,
}

fn parse_line(line: &str, regex: &Regex) -> Result<Line> {
    let mut kind: Option<String> = None;
    let mut value: Option<String> = None;
    let mut indent: usize = 0;
    let mut comment: Option<String> = None;

    let captures = try!(regex.captures(line)
                             .ok_or_else(|| Error::from_str(&format!("Can't parse: {}", line))));

    for (name, actual_value) in captures.iter_named() {
        match name {
            "indent" => {
                indent = actual_value.map_or(0, |value| value.len());
            }

            "comment" => {
                comment = actual_value.map(|value| value.to_owned());
            }

            _ => {
                match actual_value {
                    None => {
                        kind = Some(name.to_owned());
                        value = actual_value.map(|value| value.to_owned());
                    }

                    Some(other) => {
                        return Err(Error::from_str(&format!("Double match: {} and {}", other, name)));
                    }
                }
            }
        }
    }

    match kind {
        None => {
            unimplemented!()
        }

        Some(kind) => {
            let kind = match kind.as_str() {
                "tags" =>
                    LineKind::Tags,

                "feature" =>
                    LineKind::Feature,

                "background" =>
                    LineKind::Background,

                "scenario" =>
                    LineKind::Scenario,

                "outline" =>
                    LineKind::ScenarioOutline,

                "given" =>
                    LineKind::GivenStep,

                "when" =>
                   LineKind::WhenStep,

                "then" =>
                    LineKind::ThenStep,

                "and" =>
                    LineKind::AndStep,

                "but" =>
                    LineKind::ButStep,

                "examples" =>
                    LineKind::Examples,

                "table" =>
                    LineKind::Table,

                "docstring" =>
                    LineKind::DocString,

                "other" =>
                    LineKind::Other,

                _ => {
                    return Err(Error::from_str(&format!("Unknown match: {}", kind)));
                }
            };

            Ok(Line{
                kind: kind,
                value: value,
                indent: indent,
                comment: comment
            })
        }
    }
}
