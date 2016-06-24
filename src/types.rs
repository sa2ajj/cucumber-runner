#[derive(Debug)]
pub struct Feature {
    pub filename: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub background: Option<Steps>,
    pub items: Vec<FeatureItem>,
}

#[derive(Debug)]
pub enum FeatureItem {
    Scenario(Scenario),
    ScenarioOutline(ScenarioOutline),
}

#[derive(Debug)]
pub struct Scenario {
    pub lineno: u32,
    pub name: String,
    pub description: String,
    pub steps: Steps,
}

#[derive(Debug)]
pub struct ScenarioOutline {
    pub lineno: u32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub steps: Steps,
    pub examples: Vec<Table>,
}

#[derive(Debug)]
pub enum StepKind {
    Given,
    When,
    Then,
}

#[derive(Debug)]
pub enum StepArg {
    None,
    MultiLine(String),
    Table(Table),
}

#[derive(Debug)]
pub struct Step {
    pub lineno: u32,
    pub kind: StepKind,
    pub definition: String,
    pub arg: StepArg,
}

pub type Steps = Vec<Step>;

pub type Table = Vec<Vec<String>>;

impl Feature {
}
