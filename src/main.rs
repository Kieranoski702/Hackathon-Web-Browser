mod html_adt;
mod renderer;
mod ansi_helper;

use html_adt::*;

fn main() {
    let tokens = vec![
        Token::start(Elem::OL),
            Token::start(Elem::LI),
                Token::text("Item 1"),
            Token::end(Elem::LI),
            Token::start(Elem::UL),
                Token::start(Elem::LI),
                    Token::text("Item 1.5"),
                Token::end(Elem::LI),
            Token::end(Elem::UL),
            Token::start(Elem::LI),
                Token::text("Item 2"),
            Token::end(Elem::LI),
        Token::end(Elem::OL)
    ];

    let mut r: renderer::Renderer = Default::default();

    match r.render(&tokens) {
        Ok(html) => println!("{}", html),
        Err(err) => println!("{}", err)
    }
}
