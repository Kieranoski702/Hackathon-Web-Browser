use crate::html_adt::{Attrs, Elem, Token};
use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, is_a};
use nom::character::complete::{anychar, char, multispace0, none_of};
use nom::combinator::{eof, fail, opt, peek};
use nom::multi::{many0, many1, many_till};
use nom::IResult;
use regex::Regex;
use std::borrow::Cow;

pub fn parse_html<'a>(i: &'a str) -> IResult<Cow<'a, str>, Vec<Token>> {
    let parsed = pre_process(i);
    println!("{}", parsed);
    match parse_inner(&parsed) {
        Ok((_, tokens)) => Ok((Cow::from(i), tokens)),
        Err(_) => fail(Cow::from(i))
    }
}

fn pre_process(i: &str) -> Cow<'_, str> {
    lazy_static! {
        static ref REMOVE_RE: Regex = Regex::new(r"(?:(?s)<!--.*?-->)|<![^>]*?>").unwrap();
    }

    REMOVE_RE.replace_all(i, "")
}

/**
 * Parse a HTML file into a HTML object.
 */
fn parse_inner<'a>(i: &'a str) -> IResult<&'a str, Vec<Token>> {
    // println!("START: p_html {} ", i);
    let (i, _) = opt(|n| p_open_tag_by_elem(Elem::HTML, n))(i)?;
    let (i, _) = opt(|n| p_skip_tag_by_elem(Elem::HEAD, n))(i)?;
    let (i, _) = opt(|n| p_open_tag_by_elem(Elem::BODY, n))(i)?;
    let (i, body) = p_body(i)?;
    let (i, _) = opt(|n| p_close_tag_by_elem(Elem::BODY, n))(i)?;
    let (i, _) = opt(|n| p_close_tag_by_elem(Elem::HTML, n))(i)?;
    let (i, _) = eof(i)?;
    return Ok((i, body));
}

fn p_body(i: &str) -> IResult<&str, Vec<Token>> {
    // println!("START: p_body {} ", i);
    let (i, content) = many1(alt((p_skip,p_elem, p_text)))(i)?;

    // Oh no , content is a Vec<Vec<Token>>
    // https://users.rust-lang.org/t/flatten-a-vec-vec-t-to-a-vec-t/24526
    // println!("OK: p_body {} ", i);
    return Ok((i, content.into_iter().flatten().collect()));
}

fn p_text(i: &str) -> IResult<&str, Vec<Token>> {
    // println!("START: p_text {}", i);
    // parse until i find an element tag
    let (i, f) = multispace0(i)?;

    let (i, (text, _)) = many_till(anychar, peek(alt((p_close_tag, p_open_tag))))(i)?;

    let s: String = text.iter().collect();
    let f: String = f.chars().collect();
    if s.len() == 0 && f.len() == 0 {
        return fail("foo");
    }
    let (i, _) = multispace0(i)?;
    let mut vec: Vec<Token> = Vec::new();
    let token = Token::TEXT(s);
    let token2 = Token::clone(&token);
    // println!("VALID TEXT: {:#?}", &token2);
    // println!("OK: p_text {}", i);
    vec.push(token);

    return Ok((i, vec));
}


fn p_skip(i:&str) -> IResult<&str,Vec<Token>> {
    let (i,_) = alt((|n|p_skip_tag_by_elem(Elem::STYLE, n),
    |n|p_skip_tag_by_elem(Elem::SCRIPT, n),
    |n|p_skip_tag_by_elem(Elem::META, n)))(i)?;

    Ok((i,Vec::new()))
}

fn p_elem(i: &str) -> IResult<&str, Vec<Token>> {
    // println!("START: p_elem {}", i);
    let (i, _) = multispace0(i)?;
    let (i, start) = p_open_tag(i)?;
    // println!("ELEM FOUND open tag {:#?}", start);
    let (i, _) = multispace0(i)?;
    let (i, inner) = many0(alt((p_skip,p_elem, p_text)))(i)?;
    // println!("ELEM FOUND inner {:#?}", inner);
    let (i, _) = multispace0(i)?;
    let (i, close) = opt(|n| p_close_certain_tag(Token::clone(&start), n))(i)?;
    let (i, _) = multispace0(i)?;
    // println!("ELEM FOUND close tag {:#?}", close);

    let mut vec = Vec::new();

    let inner: Vec<Token> = inner.into_iter().flatten().collect();

    let start2 = Token::clone(&start);
    vec.push(start);
    vec.extend(inner);

    if let Some(c) = close {
        if let Token::START(_,b) = start2 {
            if let Token::END(e1, _) = c{
            let c = Token::END(e1,b);
            vec.push(c)
        }
    }
    };

    // println!("VALID TAG {:#?}", vec);
    // println!("OK: p_elem{}", i);
    return Ok((i, vec));
}

fn p_open_tag(i: &str) -> IResult<&str, Token> {
    // println!("START: p_open_tag {}", i);
    let (i, _) = multispace0(i)?;
    let (i, _) = char('<')(i)?;
    let (i, _) = multispace0(i)?;
    let (i, name) = take_while(|c: char| c.is_ascii_alphanumeric())(i)?;
    let (i, _) = multispace0(i)?;
    let (i,attrs) = p_attrs(i)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = alt((tag(">"),tag("/>")))(i)?;
    let (i, _) = multispace0(i)?;

    let elem = match_elem(name);
    match elem {
        None => {
            return fail("Unknown");
        },
        Some(e) => {
            // println!("OK: p_open_tag ({}) {}", name,i);
            return Ok((i, Token::START(e,attrs)));
        }
    }

}

fn p_close_tag(i: &str) -> IResult<&str, Token> {
    // println!("START: p_close_tag {}", i);
    let (i, _) = multispace0(i)?;
    let (i, _) = char('<')(i)?;
    let (i, _) = char('/')(i)?;
    // println!("foo1");
    let (i, _) = multispace0(i)?;
    let (i, name) = take_while(|c: char| c.is_ascii_alphanumeric())(i)?;
    // println!("foo2");
    let (i, _) = multispace0(i)?;
    let (i, _) = char('>')(i)?;
    let (i, _) = multispace0(i)?;
    // println!("foo3");

    let elem = match_elem(name);
    match elem {
        None => {
            return fail("Unknown");
        },
        Some(e) => {
            // println!("OK: p_close_tag ({}) {}", name,i);
            return Ok((i, Token::END(e, Attrs::new())));
        }
    }
}

fn p_close_certain_tag(desired: Token, i: &str) -> IResult<&str, Token> {
    let desired_elem = match desired {
        Token::START(e, _) => e,
        Token::END(e, _) => e,
        _ => todo!(),
    };

    let (i, token) = p_close_tag_by_elem(desired_elem, i)?;

    // println!("OK : close {:#?}",desired_elem);
    return Ok((i, token));
}

fn match_elem(name: &str) -> Option<Elem> {
    let a = String::from(name).to_lowercase();
    // println!("hellomatch");
    match a.as_str() {
        // Boilerplate
        "html"    => Some(Elem::HTML),
        "body"    => Some(Elem::BODY),
        "head"    => Some(Elem::HEAD),
        // Format / inline
        "b"       => Some(Elem::STRONG),
        "strong"  => Some(Elem::STRONG),
        "i"       => Some(Elem::EM),
        "u"       => Some(Elem::U),
        "em"      => Some(Elem::EM),
        "h1"      => Some(Elem::H1),
        "h2"      => Some(Elem::H2),
        "h3"      => Some(Elem::H3),
        "h4"      => Some(Elem::H4),
        "h5"      => Some(Elem::H5),
        // Sectional commands
        "header"  => Some(Elem::HEADER),
        "p"       => Some(Elem::P),
        "div"     => Some(Elem::DIV),
        "nav"     => Some(Elem::NAV),
        "main"    => Some(Elem::MAIN),
        "a"       => Some(Elem::A),
        "li"      => Some(Elem::LI),
        "ol"      => Some(Elem::OL),
        "ul"      => Some(Elem::UL),
        "section" => Some(Elem::DIV),
        "br"      => Some(Elem::BR),
        "meta" => Some(Elem::META),
        "script" => Some(Elem::SCRIPT),
        "style" => Some(Elem::STYLE),

        _ => None
    }
}

fn p_open_tag_by_elem(elem: Elem, i: &str) -> IResult<&str, Token> {
    let (i, token) = p_open_tag(i)?;
    let token_elem = match token {
        Token::START(e, _) => e,
        //Token::END(e, _) => e,
        _ => todo!(),
    };
    if token_elem.eq(&elem) {
        return Ok((i, token));
    } else {
        return fail("");
    }
}

fn p_close_tag_by_elem(elem: Elem, i: &str) -> IResult<&str, Token> {
    // println!("hello");
    let (i, token) = p_close_tag(i)?;
    // println!("hello2");
    let token_elem = match token {
        //Token::START(e, _) => e,
        Token::END(e, _) => e,
        _ => panic!(),
    };
    if token_elem.eq(&elem) {
        return Ok((i, token));
    } else {
        return fail("");
    }
}

fn p_skip_tag_by_elem(elem: Elem, i: &str) -> IResult<&str, ()> {
    println!("START: skipping ({:#?}) : {}",elem,i);
    let (i, _) = p_open_tag_by_elem(elem, i)?;

    let (i, _) = many_till(none_of(""), |n| p_close_tag_by_elem(elem, n))(i)?;
    let (i,_) = opt(|n| p_close_tag_by_elem(elem,n))(i)?;

    println!("OK: skipping ({:#?}) : {}",elem,i);
    return Ok((i, ()));
}


// https://html.spec.whatwg.org/multipage/syntax.html#attributes-2


fn p_attrs(i:&str) -> IResult<&str,Attrs> {
    // println!("START: p_attrs {}",i);
    let (i,bindings) = many0(p_attr)(i)?;

    let mut attrs = Attrs::new();
    for (k,v) in bindings.iter() {
        attrs.insert(k.to_string(),v.to_string());
    };

    return Ok((i,attrs));
}

fn p_attr(i:&str) -> IResult<&str,(String,String)> {
    // println!("START: p_attr {}",i);
    let (i,_) = multispace0(i)?;
    let (i,name) = p_attr_name(i)?;
    let (i,_) = multispace0(i)?;
    let (i,val) = alt((p_attr_with_value,p_attr_with_no_value))(i)?;
    // println!("OK: p_attr {}",i);
    return Ok((i,(name,val)));
}

fn p_attr_with_value(i:&str) -> IResult<&str,String> {
    let (i,_) = multispace0(i)?;
    let (i,_) = char('=')(i)?;
    let (i,_) = multispace0(i)?;
    let (i,val) = alt((p_double_quotes,p_single_quotes,p_unquoted))(i)?;
    let (i,_) = multispace0(i)?;

    return Ok((i,val));
}

fn p_attr_with_no_value(i:&str) -> IResult<&str,String> {
    return Ok((i,String::from("")));
}
fn p_attr_name(i:&str) -> IResult<&str,String> {
    let (i,value) = many1(none_of(" \"\'>/=`"))(i)?;
    let s : String = value.into_iter().collect();
    return Ok((i,s));
}

fn p_unquoted(i:&str) -> IResult<&str,String> {
    let (i,s) = p_string2(i)?;
    return Ok((i,s));
}

fn p_double_quotes(i:&str) -> IResult<&str,String> {
    let (i,_) = char('"')(i)?;
    let (i,s) = p_string(i)?;
    let (i,_) = char('"')(i)?;
    return Ok((i,s));
}

fn p_single_quotes(i:&str) -> IResult<&str,String> {
    let (i,_) = char('\'')(i)?;
    let (i,s) = p_string(i)?;
    let (i,_) = char('\'')(i)?;
    return Ok((i,s));
}

fn p_string(i:&str) -> IResult<&str,String> {
    let (i,s) = many1(alt((p_escapes,p_non_esc)))(i)?;
    let stwo :String = s.into_iter().collect();

    Ok((i,stwo))
}

fn p_string2(i:&str) -> IResult<&str,String> {
    let (i,s) = many1(alt((p_escapes,p_non_esc)))(i)?;
    let stwo :String = s.into_iter().collect();

    Ok((i,stwo))
}

fn p_non_esc(i:&str) -> IResult<&str,String> {
    let (i,value) = many1(none_of(">/="))(i)?;
    let (i,_) = char('\'')(i)?;
    let (i,s) = p_string(i)?;
    let s : String= value.into_iter().collect();
    return Ok((i,s));
}

fn p_non_esc2(i:&str) -> IResult<&str,String> {
    let (i,value) = many1(none_of(">/="))(i)?;
    let (i,_) = char('\'')(i)?;
    let (i,s) = p_string(i)?;
    let s : String= value.into_iter().collect();
    return Ok((i,s));
}
fn p_escapes(i:&str) -> IResult<&str,String> {
    let (i,_) = char('\\')(i)?;
    let (i,a) =  is_a("nr\"\'\"?\\")(i)?;
    return Ok((i,String::from(a)));
}

fn disco(){
    panic!()
}