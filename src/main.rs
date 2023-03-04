use clap::{Parser, Subcommand};
use nom::Finish;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
mod HTMLParser;
mod ansi_helper;
use termion::{clear, cursor, terminal_size};
mod html_adt;
mod renderer;
mod requester;
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
    let cli = Cli::parse();
    let contents: String;
    match &cli.command {
        Some(Commands::Search { url }) => {
            let url = url.clone().unwrap();
            contents = requester::request(&url).unwrap();
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
    let rendered_html = renderer::render(&parsed_html);

    let mut should_quit = false;
    while !should_quit {
        let input = read_line();
        match input.trim() {
            "quit" => should_quit = true,
            "search" => {
                let url = read_line();
                let contents = requester::request(&url).unwrap();
                let parsed_html = HTMLParser::parseHTML(&contents);
                let rendered_html = renderer::render(&parsed_html);
                reader(rendered_html);
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

fn reader(rendered_html: String) {
    let mut should_quit = false;
    let mut scroll_offset = 0;
    while !should_quit {
        // clear the terminal and move cursor to top-left
        print!("{}{}", clear::All, cursor::Goto(1, 1));
    
        // get terminal size to determine how many lines we can show
        let (term_width, term_height) = terminal_size().unwrap();
        let max_lines = term_height - 2; // leave 1 line for input prompt and 1 line for status message
    
        // print the current viewable portion of the HTML
        let html_lines = rendered_html.lines().skip(scroll_offset).take(max_lines.into());
        for line in html_lines {
            println!("{}", line);
        }
    
        // print status message and input prompt
        println!("Press 'q' to quit, 'j' to scroll down, 'k' to scroll up.");
        print!("> ");
        stdout().flush().unwrap();
    
        // read user input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
    
        // handle input
        match input.trim() {
            "q" => should_quit = true,
            "j" => {
                // scroll down
                scroll_offset += 1;
                if scroll_offset > rendered_html.lines().count() {
                    scroll_offset = rendered_html.lines().count();
                }
            },
            "k" => {
                // scroll up
                if scroll_offset > 0 {
                    scroll_offset -= 1;
                }
            },
            _ => println!("Invalid command"),
        }
    }
    return;
}
