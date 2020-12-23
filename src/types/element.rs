use std::collections::HashMap;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Element {
    pub name: String,
    pub attrs: HashMap<String, String>,
    pub content: String,
    pub children: Vec<Element>,
}
