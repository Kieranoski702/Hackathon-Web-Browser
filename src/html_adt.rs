use std::collections::HashMap;

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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    START(Elem),
    END(Elem),
    TEXT(String),
    PARAGRAPH,
}

pub struct Header {}
