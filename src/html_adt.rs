use std::collections::HashMap;

pub type Attrs = HashMap<String, String>;

pub mod attr_names {
    pub const HREF: &str = "href";
}

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Elem {
    STRONG,
    EM,
    U,
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

impl Token {
    pub fn start(e: Elem) -> Self {
        Token::START(e, Attrs::new())
    }

    pub fn end(e: Elem) -> Self {
        Token::END(e, Attrs::new())
    }

    pub fn text<T: Into<String>>(text: T) -> Self {
        Token::TEXT(text.into())
    }
}

pub struct Header {}
