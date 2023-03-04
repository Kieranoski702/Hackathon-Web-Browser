mod html_adt;
use std::fs::File;
use std::io::Read;

mod ansi_helper;
mod HTMLParser;
mod renderer;

fn main() {
    // Read the contents of the index.html file into a string
    let mut file = File::open("index.html").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    // Pass the contents of the file to the parser
    let parsed_html = HTMLParser::parseHTML(&contents);

    // Pass the parsed HTML to the renderer
    renderer::render(&parsed_html);
}
