use std::collections::HashMap;

/**
 * Root HTML structure.
 */
pub struct HTML {
    /**
     * The header of the HTML.
     */
    pub head: Header,
    /**
     * The body of the HTML.
     */
    pub body: Body,
}

/**
 * Header structure.
 */
pub struct Header {}

/**
 * Body structure.
 */
pub struct Body {
    /**
     * The Elements of the body.
     */
    pub elements: Vec<Token>
}

pub enum Token {
    START(Elem),
    END(Elem),
    TEXT(String),
    PARAGRAPH
}

pub enum Elem {
    STRONG,
    EM
}
