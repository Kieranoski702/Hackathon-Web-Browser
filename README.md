# Hackathon-Web-Browser
A html parser, renderer and terminal reader written in rust. Can search for html pages using urls or load from the local filesystem.

Please note:
Most webpages will not work as they have tags or characters that the parser cannot handle yet.


## General information

* Parser
    * HTML parser -- XML parser
* Render
* Common interface
* Requester


This project is written by **group 20**:

- hc240 - Hyochan Cho
- nd60 - Niklas Dewally
- kf77  - Kieran Fowlds


## Installation and usage

First, ensure that you have the latest version of rust and cargo installed. Please follow the instruction on the rust website (https://www.rust-lang.org/tools/install)

You can run the program in three ways:
### Using cargo run
While in the root directory of the project run the command
```bash
cargo run -- open [Filename]
```
or you can search using a url(Make sure it is a full https:// link) like so
```bash
cargo run -- search [URL]
```

### Using cargo build
While in the root directory of the project run the command
```bash
cargo build
```
This will create an executable program file called hackathon_web_browser in the target/debug directory
You can then use this like so:
```bash
./hackathon_web_browser open [Filename]
```
Or you can search like so:
```bash
./hackathon_web_browser search [URL]
```

### Using cargo build --release
Finally you can build the program in release mode which will take longer to compile but implements more optimizations on the program to make it run faster. Please note that this is not recommended simply because the speed increase isn't worth the time it takes to compile unless you want a permanent copy of the program for some reason.

To build the program in release mode simply type:
```bash
cargo build --release
```
This will create the executable file in the target/release folder and can be run in exactly the same way as the previous method

### Useful things to know
When you first run the program on a html file or url you should go into the reader mode in the terminal which has been made to mimic `less`. You can use the up and down arrow keys to scroll and you can press q to quit the reader view. 

When you quit out of reader view you will be put into a terminal interface. You can quit the program completely from here by writing `quit` or you can search for another url. To do this type `search` into the input then hit enter and another input line will appear. Put in the URL you want to search for in this input and you should be put into the reader view again unless the html page can't be rendered

There are some examples in the examples directory but note that Artificial_intelligence.html does not parse.