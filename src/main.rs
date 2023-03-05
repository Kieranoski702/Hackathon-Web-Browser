mod ansi_helper;
mod html_adt;
mod html_parser;
mod renderer;
mod requester;

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
use std::process::{Command, Stdio};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, cursor, terminal_size};

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
    Open {
        #[clap(value_parser)]
        file: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut contents = String::new();
    match &cli.command {
        Some(Commands::Search { url }) => {
            let url = url.clone().unwrap();
            contents = requester::request(&url).unwrap();
        }
        Some(Commands::Open { file }) => {
            let file = file.clone().unwrap();
            let mut file = File::open(file).unwrap();
            file.read_to_string(&mut contents).unwrap();
        }
        None => {
            // Read the contents of the index.html file into a string
            let mut file = File::open("index.html").unwrap();
            file.read_to_string(&mut contents).unwrap();
        }
    }

    // Pass the contents of the file to the parser
    let parsed_html = html_parser::parse_html(&contents);

    // Pass the parsed HTML to the renderer
    let mut r: renderer::Renderer = Default::default();
    match parsed_html {
        Ok(p) => {
            let rendered_html = r.render(&p.1);
            match rendered_html {
                Ok(html) => reader(html),
                Err(err) => reader(format!("{}",  err))
            }
        },
        Err(e) => {
            reader(format!("{}", e));
        }
    }

    let mut should_quit = false;
    while !should_quit {
        let input = read_line();
        match input.trim() {
            "quit" => should_quit = true,
            "search" => {
                let url = read_line();
                let contents = requester::request(&url).unwrap();
                let parsed_html = html_parser::parse_html(&contents);
                if let Ok(p) = parsed_html {
                    println!("{:?}", p.1);
                    let rendered_html = r.render(&p.1);
                    match rendered_html {
                        Ok(html) => reader(html),
                        Err(err) => reader(format!("{}", err))
                    }
                }
            }
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
    let mut less = Command::new("less")
        .args(&["-R"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    less.stdin.as_mut().unwrap().write_all(rendered_html.as_bytes()).unwrap();
    less.wait().unwrap();
}
