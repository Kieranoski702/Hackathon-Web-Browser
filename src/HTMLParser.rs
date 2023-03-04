use nom::IResult;
use nom::error::Error;
use nom::multi::many0;
use nom::multi::many_till;
use nom::branch::alt;
use nom::character::complete::{multispace0,char,anychar};
use nom::bytes::complete::{tag,take,take_while};
use char::is_alphanumeric;
use crate::html_adt::{Elem,Token,Header};
use nom::combinator::{map,map_parser,opt};

/**
 * Parse a HTML file into a HTML object.
 */
pub fn parseHTML(i: &str) -> IResult<&str,Vec<Token>> {
    let (i,_) = opt(pHTMLOpenElem)(i)?;
    let (i,_) = opt(pHeader)(i)?;
    let (i,_) = opt(pHTMLCloseElem)(i)?;

    let (i,body) = parseBody(i);
    let (i,_) = opt(pHTMLCloseElem)(i)?;

}

fn pHeader(i: &str) -> IResult<&str,Header> {
    Error("Not Implemented");
}

fn parseBody(i: &str) -> IResult<&str,Vec<Token>> {
    let (i,content) = many0(alt((pElem,pText)))?;

    // Oh no , content is a Vec<Vec<Token>>
    // https://users.rust-lang.org/t/flatten-a-vec-vec-t-to-a-vec-t/24526
    content.into_iter().flatten().collect()
}


/* Similar to what we did in haskell! */

fn ignoreSpaces<O,F>(i:&str,f:F) -> IResult<&str,O>
    where F: Fn(&str) -> IResult<&str,O> {
        let (i,_) = multispace0(i)?;
        let (i,res) = f(i)?;
        let (i,_) = mutlispace0(i)?;

        Ok(i,res)
}


fn pHTMLOpenElem(i: &str) -> IResult<&str,Token> {
    Error("Not Implemented");
}

fn pHTMLCloseElem(i: &str) -> IResult<&str,Token> {
    Error("Not Implemented");
}

fn pText(i: &str) -> IResult<&str,Vec<Token>> {
    // parse until i find an element tag
    let (i,text) = many_till(many0(anychar),pElem)(i)?;

    Ok(Token::TEXT(text))
}

fn pElem(i: &str) -> IResult<&str,Vec<Token>> {
    let (i,start) = pOpenElem(i)?;
    let (i,inner) = alt((pElem,pText))(i)?;
    let (i,close) = pCloseCertainElem(start,i)?;

    let mut vec = Vec::new();
    vec.push(start);
    vec.extend(inner);
    vec.push(close);

    Ok(i,vec);
}

fn pOpenElem(i: &str) -> IResult<&str,Token> {
    let (i,_) = ignoreSpaces(i,char('<'))?;
    let (i,name) = ignoreSpaces(i,take_while(is_alphanumeric))?;
    let (i,_) = ignoreSpaces(i,char('>'))?;

    let elem = matchElem(name)?;

    Ok(i,Token::START(elem));
}

fn pCloseElem(i: &str) -> IResult<&str,Token> {
    let (i,_) = ignoreSpaces(i,char('<'))?;
    let (i,name) = ignoreSpaces(i,take_while(is_alphanumeric))?;
    let (i,_) = ignoreSpaces(i,char('>'))?;

    let elem = matchElem(name)?;

    Ok(i,Token::END(elem));
}


fn pCloseCertainElem(desired: Token, i: &str) -> IResult<&str,Token> {


}

fn matchElem(name: &str) -> Result<Elem>{
    let name = String::from(name).to_lowercase();
    match name {
        "b" => Ok(Elem::STRONG),
        "i" => Ok(Elem::EM),
        v => Error(format!("{} element not implemented",v))
    }
}



