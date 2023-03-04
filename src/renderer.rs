use crate::html_adt::{HTML, Token, Elem};

/**
 * Render the HTML.
 */
pub fn render(html: &HTML) {
    render_tokens(&html.body.elements);
}

pub fn render_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        match token {
            Token::START(elem) => match elem {
                Elem::STRONG => renderers::start_strong(),
                Elem::EM => todo!(),
            },
            Token::END(elem) => match elem {
                Elem::STRONG => renderers::end_strong(),
                Elem::EM => todo!(),
            },
            Token::TEXT(text) => renderers::text(text),
            Token::PARAGRAPH => renderers::paragraph()
        }
    }
}

mod renderers {
    use crate::ansi_helper;

    pub fn start_strong() {
        ansi_helper::bold_on();
    }

    pub fn end_strong() {
        ansi_helper::bold_off();
    }

    pub fn text(t: &String) {
        print!("{}", t);
    }

    pub fn paragraph() {
        println!();
    }
}

