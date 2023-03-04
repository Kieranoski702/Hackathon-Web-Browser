use std::collections::HashMap;

pub type Attrs = HashMap<String, String>;


#[derive(Clone)]
pub enum Elem {
    STRONG,
    EM,
    P
}

#[derive(Clone)]
pub enum Token{
    START(Elem),
    END(Elem),
    TEXT(String),
    PARAGRAPH
}


pub struct Header {

}
