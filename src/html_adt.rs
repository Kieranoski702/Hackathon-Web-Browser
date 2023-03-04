use std::collections::HashMap;

pub type Attrs = HashMap<String, String>;


pub enum Elem {
    STRONG,
    EM,
    P
}

pub enum Token{
    START(Elem),
    END(Elem),
    TEXT(String),
    PARAGRAPH
}


pub struct Header {

}
