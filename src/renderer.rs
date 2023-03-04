use crate::html_adt::{attr_names, Attrs, Elem, Token};

use crate::ansi_helper;
use crate::ansi_helper::colours;

#[derive(Default)]
pub struct Renderer {
    _list_stack: Vec<(Elem, usize)>,
}

impl Renderer {
    pub fn render(&mut self, tokens: &Vec<Token>) -> String {
        let mut output = String::new();

        for token in tokens {
            let token_str = match token {
                Token::START(elem, _attrs) => match elem {
                    Elem::STRONG => self.start_strong(),
                    Elem::EM => self.start_italics(),
                    Elem::H1 => self.start_h1(),
                    Elem::H2 => self.start_h2(),
                    Elem::H3 => self.start_h3(),
                    Elem::H4 | Elem::H5 => self.start_small_head(),
                    Elem::A => self.start_a(),
                    Elem::P => self.nl(),
                    _ => String::new(),
                },
                Token::END(elem, attrs) => match elem {
                    Elem::STRONG => self.end_strong(),
                    Elem::EM => self.end_italics(),
                    Elem::H1 => self.end_h1(),
                    Elem::H2 => self.end_h2(),
                    Elem::H3 => self.end_h3(),
                    Elem::H4 | Elem::H5 => self.end_small_head(),
                    Elem::A => self.end_a(attrs),
                    Elem::DIV => self.nl(),
                    _ => String::new(),
                },
                Token::TEXT(text) => String::clone(text),
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
            "{}{}\n",
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

    fn start_h2(&self) -> String {
        ansi_helper::set_fg_colour(&colours::BLUE)
    }

    fn end_h2(&self) -> String {
        format!("{}\n", ansi_helper::reset_fg_colour())
    }

    fn start_h3(&self) -> String {
        ansi_helper::set_fg_colour(&colours::GREEN)
    }

    fn end_h3(&self) -> String {
        format!("{}\n", ansi_helper::reset_fg_colour())
    }

    fn start_small_head(&self) -> String {
        ansi_helper::bold_on()
    }

    fn end_small_head(&self) -> String {
        format!("{}\n", ansi_helper::bold_off())
    }

    fn start_a(&self) -> String {
        format!("{}[", ansi_helper::underline_on())
    }

    fn end_a(&self, attrs: &Attrs) -> String {
        format!(
            " {}]{}",
            attrs
                .get(&String::from(attr_names::HREF))
                .unwrap_or(&String::new()),
            ansi_helper::underline_off()
        )
    }

    fn nl(&self) -> String {
        String::from("\n")
    }
}
