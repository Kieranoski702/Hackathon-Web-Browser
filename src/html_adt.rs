use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Elem {
    STRONG,
    EM,
    H1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Token {
    START(Elem),
    END(Elem),
    TEXT(String),
    PARAGRAPH,
}

pub struct Header {}
