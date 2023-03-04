use nom::IResult;
use nom::bytes::complete::{tag,take};
use crate::html_adt::{HTML,Header,Body};

/**
 * Parse a HTML file into a HTML object.
 */
pub fn parseHTML(input: &str) -> IResult<String,HTML> {
    //TODO
}

fn parseHead(inp: &str) -> IResult<String,Body> {
    //TODO
}

fn parseBody(inp: &str) -> IResult<String,Body> {
    //TODO
}


/* Similar to what we did in haskell! */

fn tokenize<O>(f: &Fn(&str) -> IResult<str,O>) -> Fn(&str) -> IResult<str,O> {

    let inner = |inp: &str| -> IResult<str,O> {

    multispace0(inp).and_then(f).and_then(multispace0)
    };

    return inner;

}

fn tOpenTag(inp: &str -> IResult<str,None>) -> IResult<str,None> {
    tokenize<None>(|inp|{
        char(">");
        None
    })(inp)
}

fn tCloseTag(inp: &str -> IResult<str,None>) -> IResult<str,None> {
    tokenize<None>(|inp|{
        char(">");
        None
    })(inp)
}
