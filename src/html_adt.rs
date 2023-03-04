use std::collections::HashMap;

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
    PARAGRAPH,
}


pub struct Header {

}
