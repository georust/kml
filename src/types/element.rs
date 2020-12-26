use std::collections::HashMap;

/// Generic type used for supporting elements that are extensions or not currently implemented
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Element {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub content: String,
    pub children: Vec<Element>,
}
