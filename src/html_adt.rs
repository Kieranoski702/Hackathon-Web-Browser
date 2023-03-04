use std::collections::HashMap;

pub type Attrs = HashMap<String, String>;

pub mod attr_names {
    pub const HREF: &str = "href";
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Elem {
    STRONG,
    EM,
    H1,
    H2,
    H3,
    H4,
    H5,
    HEADER,
    DIV,
    NAV,
    MAIN,
    A,
    P,
    OL,
    UL,
    LI
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    START(Elem, Attrs),
    END(Elem, Attrs),
    TEXT(String),
}

pub struct Header {}
