<<<<<<< HEAD
use crate::html_adt::{Elem, Token};

use crate::ansi_helper;
use crate::ansi_helper::colours;

#[derive(Default)]
pub struct Renderer {
    list_depth: usize,
}
=======
use crate::html_adt::{Token, Elem};

/**
 * Render the HTML.
 */
>>>>>>> origin/parser

impl Renderer {
    pub fn render(&mut self, tokens: &Vec<Token>) {
        for token in tokens {
            match token {
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
                Token::TEXT(text) => self.text(text),
                Token::PARAGRAPH => self.paragraph(),
            }
        }
    }

    fn start_strong(&self) {
        ansi_helper::bold_on();
    }

    fn end_strong(&self) {
        ansi_helper::bold_off();
    }

    fn start_italics(&self) {
        ansi_helper::italics_on();
    }

    fn end_italics(&self) {
        ansi_helper::italics_off();
    }

    fn start_h1(&self) {
        ansi_helper::bold_on();
        ansi_helper::set_fg_colour(&colours::RED);
    }

    fn end_h1(&self) {
        ansi_helper::bold_off();
        ansi_helper::reset_fg_colour();
    }

    fn text(&self, t: &String) {
        print!("{}", t);
    }

    fn paragraph(&self) {
        println!();
    }
}
