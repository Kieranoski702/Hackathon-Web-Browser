use crate::html_adt::{attr_names, Attrs, Elem, Token};

use crate::ansi_helper;
use crate::ansi_helper::colours;
use std::error::Error;

#[derive(Default)]
pub struct Renderer {
    list_stack: Vec<(Elem, usize)>,
}

impl Renderer {
    fn parse_table(tokens: &Vec<Token>) -> Result<Vec<(usize, usize)>, Box<dyn Error>> {
        let mut lengths: Vec<Vec<Vec<usize>>> = Vec::new();
        let mut table_stack: Vec<Elem> = Vec::new();

        for token in tokens {
            match token {
                Token::START(elem, _) => {
                    match elem {
                        Elem::TABLE => {
                            if !table_stack.last().is_none() {
                                return Err("Must not be in table to have <table>")?
                            }
                            table_stack.push(Elem::TABLE);
                            lengths.push(Vec::new());
                        },
                        Elem::TBODY => {
                            if let Some(l) = table_stack.last() {
                                if *l != Elem::TABLE {
                                    return Err("Can't have <tbody> outside <table>.")?;
                                }
                            } else {
                                return Err("Can't have <tbody> outside <table>.")?
                            }
                            table_stack.push(Elem::TBODY);
                            lengths.last_mut().ok_or("Can't have <tbody> outside <table>.")?.push(Vec::new());
                        },
                        Elem::TR => {
                            if let Some(l) = table_stack.last() {
                                if *l != Elem::TBODY || *l != Elem::THEAD {
                                    return Err("Can't have <tr> outside <tbody>")?;
                                }
                            } else {
                                return Err("Can't have <tr> outside <tbody>")?;
                            }
                            table_stack.push(Elem::TR);
                            lengths.last_mut().ok_or("Error")
                        }
                        _ => {}
                    }
                },
                Token::END(elem, _) => {
                    match elem {
                        Elem::TABLE => {
                            table_stack.pop();
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        todo!()
    }

    pub fn render(&mut self, tokens: &Vec<Token>) -> Result<String, Box<dyn Error>> {
        let mut output = String::new();

        for token in tokens {
            let token_str = match token {
                Token::START(elem, _attrs) => match elem {
                    Elem::STRONG => self.start_strong(),
                    Elem::EM => self.start_italics(),
                    Elem::U => self.start_underline(),
                    Elem::H1 => self.start_h1(),
                    Elem::H2 => self.start_h2(),
                    Elem::H3 => self.start_h3(),
                    Elem::H4 | Elem::H5 => self.start_small_head(),
                    Elem::A => self.start_a(),
                    Elem::P => self.nl(),
                    Elem::UL | Elem::OL => self.start_list(*elem),
                    Elem::LI => self.start_li()?,
                    _ => String::new(),
                },
                Token::END(elem, attrs) => match elem {
                    Elem::STRONG => self.end_strong(),
                    Elem::EM => self.end_italics(),
                    Elem::U => self.end_underline(),
                    Elem::H1 => self.end_h1(),
                    Elem::H2 => self.end_h2(),
                    Elem::H3 => self.end_h3(),
                    Elem::H4 | Elem::H5 => self.end_small_head(),
                    Elem::A => self.end_a(attrs),
                    Elem::P => self.nl(),
                    Elem::DIV => self.nl(),
                    Elem::UL | Elem::OL => self.end_list(),
                    Elem::LI => self.nl(),
                    _ => String::new(),
                },
                Token::TEXT(text) => String::clone(text),
            };

            output.push_str(token_str.as_str());
        }

        output.push_str(&ansi_helper::reset_all());

        Ok(output.trim_end().to_string())
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

    fn start_underline(&self) -> String {
        ansi_helper::underline_on()
    }

    fn end_underline(&self) -> String {
        ansi_helper::underline_off()
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
            "{}{}\n",
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

    fn start_list(&mut self, elem: Elem) -> String {
        self.list_stack.push((elem, 1));
        // If this isn't the only list open.
        if self.list_stack.len() > 1 {
            self.nl()
        } else {
            String::new()
        }
    }

    fn end_list(&mut self) -> String {
        self.list_stack.pop();
        // If it's the last list don't change the length.
        if self.list_stack.len() == 0 {
            String::new()
        } else {
            // Need to move up a line to remove the trailing nl for the end_li.
            ansi_helper::move_up_lines(1)
        }
    }

    fn start_li(&mut self) -> Result<String, Box<dyn Error>> {
        if let Some(last_elem) = self.list_stack.last().cloned() {
            let spaces = "  ".repeat(self.list_stack.len());
            match last_elem {
                (Elem::OL, i) => {
                    self.list_stack.pop();
                    self.list_stack.push((Elem::OL, i + 1));
                    Ok(format!("{}{}. ", spaces, i))
                }
                (Elem::UL, _) => Ok(format!("{}• ", spaces)),
                _ => Err("Invalid elem in list stack")?,
            }
        } else {
            Err("Invalid <li> tag. Not in list.")?
        }
    }

    fn nl(&self) -> String {
        String::from("\n")
    }
}
