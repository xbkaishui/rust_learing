use std::{str::FromStr, collections::HashMap};

use clap::{Parser};
use anyhow::{anyhow, Result};
use reqwest::{header, Url, Client, Request, Response};
use colored::Colorize;
use mime::Mime;

use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

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
    #[clap(parse(try_from_str = parse_url))]
    url:String,
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
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

impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
         // 使用 = 进行 split，这会得到一个迭代器
         let mut split = s.split('=');
         let err = || anyhow!(format!("Failed to parse {}", s));
         Ok(Self {
             // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
             // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
             k: (split.next().ok_or_else(err)?).to_string(),
             // 从迭代器中取第二个结果作为 value
             v: (split.next().ok_or_else(err)?).to_string(),
         })
    }    
}

fn parse_kv_pair(s: &str) -> Result<KvPair>{
    Ok(s.parse()?)
}



/// 处理 get 子命令
async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

/// 处理 post 子命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    println!();
}

/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &str) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),

        // 其它 mime type，我们就直接输出
        _ => println!("{}", body),
    }
}

fn print_syntect(s: &str, ext: &str) {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);
    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

/// 打印整个响应
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    print!("{:?}", opts);
    let client = Client::new();
    let result = match  opts.subcmd {
        SubCommand::Get(ref args) => {
            println!("run get")
        },
        SubCommand::Post(ref args) => {
            println!("run post")
        }
    };
    Ok(result)
}
