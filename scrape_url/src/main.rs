use std::fs;

fn main() {
    let url = "https://www.rust-lang.org";
    let output = "rust.md";
    println!("featching url {}", url);

    let body = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();

    println!("resp body {}", body);

    println!("convert html to md ");

    let md = html2md::parse_html(&body);
    fs::write(output, md.as_bytes());
}
