mod ansi_helper;
mod html_adt;
mod html_parser;
mod renderer;
mod requester;

use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{stdin, stdout, Read, Write};
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
    if let Ok(p) = parsed_html {
        println!("{:?}", p.1);
        let rendered_html = r.render(&p.1);
        reader(rendered_html);
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
                    reader(rendered_html);
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
    let mut scroll_offset = 0;

    let stdin = stdin();
    let mut events = stdin.events();

    // enable raw mode to read events without waiting for enter key
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();

    loop {
        // clear the terminal and move cursor to top-left
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        // get terminal size to determine how many lines we can show
        let (_term_width, term_height) = terminal_size().unwrap();
        let max_lines = term_height - 2; // leave 1 line for input prompt and 1 line for status message

        // print the current viewable portion of the HTML
        let html_lines = rendered_html
            .lines()
            .skip(scroll_offset)
            .take(max_lines.into());
        for line in html_lines {
            writeln!(stdout, "{}", line).unwrap();
            write!(stdout, "\r").unwrap();
        }

        // print status message and input prompt
        writeln!(stdout, "Press 'q' to quit, up/down arrow keys to scroll.\r").unwrap();
        writeln!(stdout, "> \r").unwrap();
        stdout.flush().unwrap();

        // read user input events
        if let Some(Ok(event)) = events.next() {
            match event {
                Event::Key(key) => match key {
                    Key::Char('q') => {
                        break;
                    }
                    Key::Up => {
                        // scroll up
                        if scroll_offset > 0 {
                            scroll_offset -= 1;
                        }
                    }
                    Key::Down => {
                        // scroll down
                        scroll_offset += 1;
                        if scroll_offset > rendered_html.lines().count() {
                            scroll_offset = rendered_html.lines().count();
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }

    // disable raw mode before returning
    write!(stdout, "{}", cursor::Show).unwrap();
    stdout.flush().unwrap();
    drop(stdout);

    return;
}
