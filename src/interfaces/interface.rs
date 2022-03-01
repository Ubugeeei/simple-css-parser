// use super::dom::{Node, NodeType};
use combine::error::StringStreamError;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

impl Stylesheet {
    pub fn new(rules: Vec<Rule>) -> Self {
        Stylesheet { rules: rules }
    }
}

#[derive(Debug, PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

pub type Selector = SimpleSelector;

#[derive(Debug, PartialEq)]
pub enum SimpleSelector {
    UniversalSelector,
    TypeSelector {
        tag_name: String,
    },
    AttributeSelector {
        tag_name: String,
        op: AttributeSelectorOp,
        attribute: String,
        value: String,
    },
    ClassSelector {
        class_name: String,
    },
}

#[derive(Debug, PartialEq)]
pub enum AttributeSelectorOp {
    Eq,      // =
    Contain, // ~=
}

#[derive(Debug, PartialEq)]
pub struct Declaration {
    pub name: String,
    pub value: CSSValue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CSSValue {
    Keyword(String),
    Length((usize, Unit)),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Unit {
    Em,
}

#[derive(Error, Debug, PartialEq)]
pub enum CSSParseError {
    #[error("failed to parse; {0}")]
    InvalidResourceError(StringStreamError),
}
