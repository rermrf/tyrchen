use clap::{Subcommand, Parser};

#[derive(Parser, Debug)]
#[command(author, version)]
struct Options {
    #[command(subcommand)]
    subcmd: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand{
    Get(Get),
    Post(POst),
}

#[derive(Parser, Debug)]
struct Get{
    url: String,
}

#[derive(Parser, Debug)]
struct POst{
    url: String,
    body: Vec<String>,
}

fn main() {
    
    println!("Hello, world!");
}
