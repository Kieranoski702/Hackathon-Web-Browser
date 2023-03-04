mod html_adt;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
mod ansi_helper;
use clap::{Parser, Subcommand};
mod HtmlParser;
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

    // Pass the parsed HTML to the renderer
    renderer::render(&parsed_html);

    let mut should_quit = false;
    while !should_quit {
        let input = read_line();
        match input.trim() {
            "quit" => should_quit = true,
            "search" => {
                let url = read_line();
                let contents = Requester::request(&url).unwrap();
                let parsed_html = HTMLParser::parseHTML(&contents);
                renderer::render(&parsed_html);
            },
            _ => println!("Invalid command"),
        }
    }
}

fn read_line() -> String {
    let mut input = String::new();
    print!("> ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    input
}
