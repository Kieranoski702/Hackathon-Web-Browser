use crate::html_adt::{Elem, Token};

use crate::ansi_helper;
use crate::ansi_helper::colours;

#[derive(Default)]
pub struct Renderer {
    list_stack: Vec<(Elem, usize)>,
}

impl Renderer {
    pub fn render(&mut self, tokens: &Vec<Token>) -> String {
        let mut output = String::new();

        for token in tokens {
            let token_str = match token {
                Token::START(elem) => match elem {
                    Elem::STRONG => self.start_strong(),
                    Elem::EM => self.start_italics(),
                    Elem::H1 => self.start_h1(),
                },
                Token::END(elem) => match elem {
                    Elem::STRONG => self.end_strong(),
                    Elem::EM => self.end_italics(),
                    Elem::H1 => self.end_h1(),
                },
                Token::TEXT(text) => String::clone(text),
                Token::PARAGRAPH => self.paragraph(),
            };

            output.push_str(token_str.as_str());
        }

        output
    }

    fn start_strong(&self) -> String {
        ansi_helper::bold_on()
    }

    fn end_strong(&self) -> String {
        ansi_helper::bold_off()
    }

    fn start_italics(&self) -> String {
        ansi_helper::italics_on()
    }

    fn end_italics(&self) -> String {
        ansi_helper::italics_off()
    }

    fn start_h1(&self) -> String {
        format!(
            "{}{}",
            ansi_helper::bold_on(),
            ansi_helper::set_fg_colour(&colours::RED)
        )
    }

    fn end_h1(&self) -> String {
        format!(
            "{}{}",
            ansi_helper::bold_off(),
            ansi_helper::reset_fg_colour()
        )
    }

    fn paragraph(&self) -> String {
        return String::from("\n");
    }
}
