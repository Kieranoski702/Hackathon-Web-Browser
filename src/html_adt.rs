use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Elem {
    STRONG,
    EM,
    H1,
}

#[derive(Clone, Debug)]
pub enum Token {
    START(Elem),
    END(Elem),
    TEXT(String),
    PARAGRAPH,
}

pub struct Header {}
