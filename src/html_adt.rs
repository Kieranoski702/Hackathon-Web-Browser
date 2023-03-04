pub enum Token {
    START(Elem),
    END(Elem),
    TEXT(String),
    PARAGRAPH,
}

pub enum Elem {
    STRONG,
    EM,
    H1,
}
