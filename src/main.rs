mod html_adt;
use std::fs::File;
use std::io::Read;

mod ansi_helper;
use clap::{Parser, Subcommand};
mod HTMLParser;
mod renderer;
mod Requester;

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
    let cli = Cli::parse();
    let contents: String;
    match &cli.command {
        Some(Commands::Search { url }) => {
            let url = url.clone().unwrap();
            contents = Requester::request(&url).unwrap();
        }
        None => {
            // Read the contents of the index.html file into a string
            let mut file = File::open("index.html").unwrap();
            file.read_to_string(&mut contents).unwrap();
        }
    }

    // Pass the contents of the file to the parser
    let parsed_html = HTMLParser::parseHTML(&contents);

    match parsed_html  {
        Ok((_,p)) => renderer::render_tokens(&p),
        _ => todo!()
    }

    // Pass the parsed HTML to the renderer
}
