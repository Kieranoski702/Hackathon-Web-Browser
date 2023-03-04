use std::collections::HashMap;

pub type Attrs = HashMap<String, String>;

pub struct HTML {
    head: Header,
    body: Body,
}

pub struct Header {}

pub struct Body {
    elements: Vec<Elem>
}


pub enum Elem {
    STRONG(Vec<Elem>, Attrs),
    EM(Vec<Elem>, Attrs),
    P(Vec<Elem>, Attrs),
    TEXT(String)
}
