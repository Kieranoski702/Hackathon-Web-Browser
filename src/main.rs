use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::Read;
use nom::Finish;

mod HTMLParser;
mod Requester;
mod ansi_helper;
mod html_adt;
mod renderer;

use html_adt::*;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for the given url on the internet
    Search {
        #[clap(value_parser)]
        url: Option<String>,
    },
}

fn main() {
    // let cli = Cli::parse();
    // let contents: String;
    // match &cli.command {
    //     Some(Commands::Search { url }) => {
    //         let url = url.clone().unwrap();
    //         contents = Requester::request(&url).unwrap();
    //     }
    //     None => {
    //         // Read the contents of the index.html file into a string
    //         let mut file = File::open("index.html").unwrap();
    //         file.read_to_string(&mut contents).unwrap();
    //     }
    // }

    // // Pass the contents of the file to the parser
    // let parsed_html = HTMLParser::parseHTML(&contents);

    // // Pass the parsed HTML to the renderer
    // renderer::render(&parsed_html);

    // // Pass the contents of the file to the parser
    // let parsed_html = HTMLParser::parseHTML(&contents);

    // // Pass the parsed HTML to the renderer
    // renderer::render(&parsed_html);

    let html = "<b>Hello World!</b>";

    let parsed = HTMLParser::parseHTML(html).finish();

    let mut r: renderer::Renderer = Default::default();

    if let Ok(p) = parsed {
        println!("{:?}", p.1);

        r.render(&p.1);
    } else {
        println!("Err");
    }


}
