use std::collections::HashMap;

pub type Attrs = HashMap<String, String>;

pub mod attr_names {
    pub const HREF: &str = "href";
}

#[derive(Clone, Debug)]
pub enum Elem {
    STRONG,
    EM,
    H1,
    H2,
    DIV,
    A,
}

#[derive(Clone, Debug)]
pub enum Token {
    START(Elem, Attrs),
    END(Elem, Attrs),
    TEXT(String),
    PARAGRAPH,
}

pub struct Header {}
