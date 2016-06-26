// this is not a tokenizer in the classical sense (imho, at least)

use regex::{
    Regex,
    RegexBuilder,
};

use super::Error;
use super::Result;

// nay be used only once and we ignore it
pub static LANGUAGE_RE: &'static str = r"(Language:\s*(?P<language>\S+))";

pub static TAGS_RE: &'static str = r"(?P<tags>@\w+(\s+@\w+)*)";

pub static FEATURE_RE: &'static str = r"(Feature:\s*(?P<feature>.*))";
pub static BACKGROUND_RE: &'static str = r"(Background:\s*(?P<background>.*))";
pub static SCENARIO_RE: &'static str = r"(Scenario:\s*(?P<scenario>.*))";
pub static SCENARIO_OUTLINE_RE: &'static str = r"(Scenario Outline:\s*(?P<outline>.*))";
pub static GIVEN_STEP_RE: &'static str = r"(Given\s+(?P<given>.*))";
pub static WHEN_STEP_RE: &'static str = r"(When\s+(?P<when>.*))";
pub static THEN_STEP_RE: &'static str = r"(Then\s+(?P<then>.*))";
pub static AND_STEP_RE: &'static str = r"(And\s+(?P<and>.*))";
pub static BUT_STEP_RE: &'static str = r"(But\s+(?P<but>.*))";
pub static EXAMPLES_RE: &'static str = r"(Examples:\s*(?P<examples>.*))";

pub static TABLE_RE: &'static str = r"(?P<table>\|.*\|)";
pub static DOCSTRING_RE: &'static str = "(?P<docstring>\"\"\"|''')";

pub static OTHER_RE: &'static str = r"(?P<other>[^\#]+)";

pub static INDENT_RE: &'static str = r"(?P<indent>\s*)";
// '#' requires escaping because of the way we compile regex
pub static COMMENT_RE: &'static str = r"\s*(\#\s*(?P<comment>.*))?";

pub fn prepare_regex(bits: &[&str], detect_comment: bool) -> Regex {
    let actual_bits = &bits.join("|");
    let mut pattern: Vec<&str> = vec!("^",
                                      INDENT_RE,
                                      "(",
                                      &actual_bits,
                                      ")?");
    if detect_comment {
        pattern.push(COMMENT_RE);
    }

    pattern.push("$");

    let tempo = pattern.join(r"");

    println!("prepare_regex: {:?}", tempo);

    RegexBuilder::new(&pattern.join(r""))
                 .case_insensitive(true)
                 .ignore_whitespace(true)
                 .compile()
                 .unwrap()
}

#[derive(Debug)]
pub enum Line {
    EmptyLine(usize, Option<String>),
    Language(usize, Option<String>, String),
    Tags(usize, Option<String>, Vec<String>),
    Feature(usize, Option<String>, Option<String>),
    Background(usize, Option<String>),
    Scenario(usize, Option<String>, Option<String>),
    ScenarioOutline(usize, Option<String>, Option<String>),
    GivenStep(usize, Option<String>, String),
    WhenStep(usize, Option<String>, String),
    ThenStep(usize, Option<String>, String),
    AndStep(usize, Option<String>, String),
    ButStep(usize, Option<String>, String),
    Examples(usize, Option<String>),
    TableRow(usize, Option<String>, Vec<String>),
    DocStringDelimiter(usize, Option<String>, String),
    Other(usize, Option<String>, String),
}

pub fn parse_line(line: &str, regex: &Regex) -> Result<Line> {
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
                // ignore None results
                if let Some(actual_value) = actual_value {
                    if let Some(other) = kind {
                        return Err(Error::from_str(&format!("Double match: {} and {}", other, name)));
                    }

                    kind = Some(name.to_owned());
                    value = Some(actual_value.to_owned());
                }
            }
        }
    }

    Ok(match kind.unwrap_or(String::new()).as_str() {
        "" =>
            Line::EmptyLine(indent, comment),

        "language" =>
            Line::Language(indent, comment, value.unwrap()),

        "tags" =>
            Line::Tags(indent, comment, value.map_or(Vec::new(),
                                                     |tags| tags.split(' ')
                                                                .map(|tag| tag.to_owned())
                                                                .collect())),
        "feature" =>
            Line::Feature(indent, comment, value),

        "background" =>
            Line::Background(indent, comment),

        "scenario" =>
            Line::Scenario(indent, comment, value),

        "outline" =>
            Line::ScenarioOutline(indent, comment, value),

        "given" =>
            Line::GivenStep(indent, comment, value.unwrap()),

        "when" =>
            Line::WhenStep(indent, comment, value.unwrap()),

        "then" =>
            Line::ThenStep(indent, comment, value.unwrap()),

        "and" =>
            Line::AndStep(indent, comment, value.unwrap()),

        "but" =>
            Line::ButStep(indent, comment, value.unwrap()),

        "examples" =>
            Line::Examples(indent, comment),

        "table" => {
            let value = value.unwrap();
            let rows: Vec<&str> = value.split('|').collect();

            // XXX: check the content
            Line::TableRow(indent, comment, rows.into_iter()
                                                .map(|item| item.to_owned())
                                                .collect())
        }

        "docstring" =>
            Line::DocStringDelimiter(indent, comment, value.unwrap()),

        "other" =>
            Line::Other(indent, comment, value.unwrap()),

        kind @ _ => {
            return Err(Error::from_str(&format!("Unknown kind: {}", kind)));
        }
    })
}
