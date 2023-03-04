use std::collections::HashMap;

pub type Attrs = HashMap<String, String>;

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
    pub elements: Vec<Elem>
}

/**
 * Element types.
 */
pub enum Elem {
    /**
     * b / strong tags.
     */
    STRONG(Vec<Elem>, Attrs),
    /**
     * i / em tags.
     */
    EM(Vec<Elem>, Attrs),
    /**
     * p tags.
     */
    P(Vec<Elem>, Attrs),
    /**
     * Text blocks.
     */
    TEXT(String)
}
