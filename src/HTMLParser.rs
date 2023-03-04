use crate::html_adt::{Elem, Header, Token};
use nom::branch::alt;
use nom::bytes::complete::{tag, take, take_while};
use nom::character::complete::{anychar, char, multispace0};
use nom::character::is_alphanumeric;
use nom::combinator::{map, map_parser, opt};
use nom::error::{Error, ErrorKind};
use nom::multi::many0;
use nom::multi::many_till;
use nom::IResult;

/**
 * Parse a HTML file into a HTML object.
 */
pub fn parseHTML<'a>(i: &'a str) -> IResult<&'a str, Vec<Token>> {
    // let (i, _) = opt(pHTMLOpenElem)(i)?;
    // let (i, _) = opt(pHeader)(i)?;
    // let (i, _) = opt(pHTMLCloseElem)(i)?;

    let (i, body) = parseBody(i)?;
    // let (i, _) = opt(pHTMLCloseElem)(i)?;

    Ok((i, body))
}

fn pHeader(i: &str) -> IResult<&str, Header> {
    todo!();
}

fn parseBody(i: &str) -> IResult<&str, Vec<Token>> {
    let (i, mut content) = many0(alt((pElem, pText)))(i)?;

    // Oh no , content is a Vec<Vec<Token>>
    // https://users.rust-lang.org/t/flatten-a-vec-vec-t-to-a-vec-t/24526
    Ok((i, content.into_iter().flatten().collect()))
}

/* Similar to what we did in haskell! */

fn ignoreSpaces<O, F>(i: &str, f: F) -> IResult<&str, O>
where
    F: Fn(&str) -> IResult<&str, O>,
{
    let (i, _) = multispace0(i)?;
    let (i, res) = f(i)?;
    let (i, _) = multispace0(i)?;

    Ok((i, res))
}

fn pHTMLOpenElem(i: &str) -> IResult<&str, Token> {
    todo!();
}

fn pHTMLCloseElem(i: &str) -> IResult<&str, Token> {
    todo!();
}

fn pText(i: &str) -> IResult<&str, Vec<Token>> {
    // parse until i find an element tag
    let (i, (text, _)) = many_till(anychar, pElem)(i)?;

    let s = text.iter().collect();
    let mut vec: Vec<Token> = Vec::new();
    let token = Token::TEXT(s);
    vec.push(token);

    Ok((i, vec))
}

fn pElem(i: &str) -> IResult<&str, Vec<Token>> {
    let (i, start) = pOpenElem(i)?;
    let (i, inner) = alt((pElem, pText))(i)?;
    let (i, close) = pCloseCertainElem(Token::clone(&start), i)?;

    let mut vec = Vec::new();
    vec.push(start);
    vec.extend(inner);
    vec.push(close);

    Ok((i, vec))
}

fn pOpenElem(i: &str) -> IResult<&str, Token> {
    let (i, _) = multispace0(i)?;
    let (i, _) = char('<')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, name) = take_while(|c: char| c.is_ascii_alphanumeric())(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char('>')(i)?;

    let elem = matchElem(name);

    Ok((i, Token::START(elem)))
}

fn pCloseElem(i: &str) -> IResult<&str, Token> {
    let (i, _) = multispace0(i)?;
    let (i, _) = char('<')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, name) = take_while(|c: char| c.is_ascii_alphanumeric())(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = tag(r"\/>")(i)?;
    let elem = matchElem(name);

    Ok((i, Token::END(elem)))
}

fn pCloseCertainElem(desired: Token, i: &str) -> IResult<&str, Token> {
    todo!()
}

fn matchElem(name: &str) -> Elem {
    let name = String::from(name).to_lowercase();
    match name.as_str() {
        "b" => Elem::STRONG,
        "i" => Elem::EM,
        v => todo!(),
    }
}
