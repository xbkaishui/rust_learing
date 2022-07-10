use clap::{Parser};
use anyhow::Result;
use reqwest::Url;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author="John")]
struct Opts{
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum  SubCommand {
    Get(Get),
    Post(Post),
}

#[derive(Parser, Debug)]
struct Get {
    // #[clap(value_parser = parse_url)]
    #[clap(parse(try_from_str = parse_url))]
    url:String,
}

#[derive(Parser, Debug)]
struct Post {
    url:String,
    body: Vec<String>,
}

#[derive(Parser, Debug)]
struct KvPair {
    k:String,
    v:String,
}

fn parse_url(url: &str) -> Result<String> {
    println!("in parse url");
    let _url:Url = url.parse()?;
    println!("{}", _url);
    Ok(url.into())
    // let errorInfo = "parse url failed";
    // return Err("Some error message");
}

fn main() {
    let opts: Opts = Opts::parse();
    print!("{:?}", opts);
}
