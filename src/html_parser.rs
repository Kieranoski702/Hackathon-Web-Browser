use crate::html_adt::{Attrs, Elem, Header, Token};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{anychar, char, multispace0};
use nom::combinator::{fail, peek};
use nom::multi::{many0, many_till};
use nom::IResult;

/**
 * Parse a HTML file into a HTML object.
 */
pub fn parse_html<'a>(i: &'a str) -> IResult<&'a str, Vec<Token>> {
    //println!("START: p_html {} ", i);
    // let (i, _) = opt(pHTMLOpenElem)(i)?;
    // let (i, _) = opt(pHeader)(i)?;
    // let (i, _) = opt(pHTMLCloseElem)(i)?;

    let (i, body) = p_body(i)?;
    // let (i, _) = opt(pHTMLCloseElem)(i)?;

    //println!("OK: p_html {} ", i);
    Ok((i, body))
}

#[allow(dead_code)]
fn p_header(i: &str) -> IResult<&str, Header> {
    todo!();
}

fn p_body(i: &str) -> IResult<&str, Vec<Token>> {
    //println!("START: p_body {} ", i);
    let (i, content) = many0(alt((p_elem, p_text)))(i)?;

    // Oh no , content is a Vec<Vec<Token>>
    // https://users.rust-lang.org/t/flatten-a-vec-vec-t-to-a-vec-t/24526
    //println!("OK: p_body {} ", i);
    Ok((i, content.into_iter().flatten().collect()))
}

/* Similar to what we did in haskell! */
#[allow(dead_code)]
fn ignore_spaces<O, F>(i: &str, f: F) -> IResult<&str, O>
where
    F: Fn(&str) -> IResult<&str, O>,
{
    let (i, _) = multispace0(i)?;
    let (i, res) = f(i)?;
    let (i, _) = multispace0(i)?;

    Ok((i, res))
}

#[allow(dead_code)]
fn p_html_open_tag(i: &str) -> IResult<&str, Token> {
    todo!();
}

#[allow(dead_code)]
fn p_html_close_tag(i: &str) -> IResult<&str, Token> {
    todo!();
}

fn p_text(i: &str) -> IResult<&str, Vec<Token>> {
    //println!("START: p_text {}", i);
    // parse until i find an element tag
    let (i, _) = multispace0(i)?;
    let (i, (text, _)) = many_till(anychar, peek(alt((p_close_tag, p_open_tag))))(i)?;

    let s: String = text.iter().collect();
    if s.len() == 0 {
        return fail("foo");
    }
    let (i, _) = multispace0(i)?;
    let mut vec: Vec<Token> = Vec::new();
    let token = Token::TEXT(s);
    // let token2 = Token::clone(&token);
    //println!("VALID TEXT: {:#?}", &token2);
    //println!("OK: p_text {}", i);
    vec.push(token);

    Ok((i, vec))
}

fn p_elem(i: &str) -> IResult<&str, Vec<Token>> {
    //println!("START: p_elem {}", i);
    let (i, _) = multispace0(i)?;
    let (i, start) = p_open_tag(i)?;
    //println!("ELEM FOUND open tag {:#?}", start);
    let (i, _) = multispace0(i)?;
    let (i, inner) = many0(alt((p_elem, p_text)))(i)?;
    //println!("ELEM FOUND inner {:#?}", inner);
    let (i, _) = multispace0(i)?;
    let (i, close) = p_close_certain_tag(Token::clone(&start), i)?;
    let (i, _) = multispace0(i)?;
    //println!("ELEM FOUND close tag {:#?}", close);

    let mut vec = Vec::new();

    let inner: Vec<Token> = inner.into_iter().flatten().collect();
    vec.push(start);
    vec.extend(inner);
    vec.push(close);
    //println!("VALID TAG {:#?}", vec);
    //println!("OK: p_elem{}", i);
    Ok((i, vec))
}

fn p_open_tag(i: &str) -> IResult<&str, Token> {
    //println!("START: p_open_tag {}", i);
    let (i, _) = multispace0(i)?;
    let (i, _) = char('<')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, name) = take_while(|c: char| c.is_ascii_alphanumeric())(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char('>')(i)?;
    let (i, _) = multispace0(i)?;

    let elem = match_elem(name);

    //println!("OK: p_open_tag {}", i);
    Ok((i, Token::START(elem, Attrs::new())))
}

fn p_close_tag(i: &str) -> IResult<&str, Token> {
    //println!("START: p_close_tag {}", i);
    let (i, _) = multispace0(i)?;
    let (i, _) = tag("</")(i)?;
    let (i, _) = multispace0(i)?;
    let (i, name) = take_while(|c: char| c.is_ascii_alphanumeric())(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = char('>')(i)?;
    let (i, _) = multispace0(i)?;

    let elem = match_elem(name);

    //println!("OK: p_close_tag {}", i);
    Ok((i, Token::END(elem, Attrs::new())))
}

fn p_close_certain_tag(desired: Token, i: &str) -> IResult<&str, Token> {
    //println!("START: p_close_certain_tag {}", i);
    let (i, token) = p_close_tag(i)?;

    let desired_elem = match desired {
        Token::START(e, _) => e,
        Token::END(e, _) => e,
        _ => todo!(),
    };
    let token_elem = match token {
        Token::START(e, _) => e,
        Token::END(e, _) => e,
        _ => todo!(),
    };
    if token_elem.eq(&desired_elem) {
        //println!("OK: p_close_certain_tag {}", i);
        Ok((i, token))
    } else {
        //println!("FAIL: p_close_certain_tag {}", i);

        fail("mismached element")
    }
}

fn match_elem(name: &str) -> Elem {
    let a = String::from(name).to_lowercase();
    ////println!("hellomatch");
    match a.as_str() {
        // Format / inline
        "b" => Elem::STRONG,
        "strong" => Elem::STRONG,
        "i" => Elem::EM,
        "em" => Elem::EM,
        "h1" => Elem::H1,
        "h2" => Elem::H2,
        "h3" => Elem::H3,
        "h4" => Elem::H4,
        "h5" => Elem::H5,
        // Sectional commands
        "header" => Elem::HEADER,
        "div" => Elem::DIV,
        "nav" => Elem::NAV,
        "main" => Elem::MAIN,

        _ => unimplemented!("HTML tag {} not implemented", a),
    }
}
