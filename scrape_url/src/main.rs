use reqwest::Url;
use std::fs;
use structopt::StructOpt;

/// use this tool to fetch html from specified url and convert to a file in markdown format
#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    /// the url to fetch
    #[structopt(long)]
    url: String,
    /// name of the markdown file content write to
    #[structopt(long, default_value = "rust.md")]
    file: String,
}

fn main() {
    let opt: Opt = Opt::from_args();

    println!("Fetching url: {}", opt.url);
    let body = reqwest::blocking::get(&opt.url).unwrap().text().unwrap();

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);
    fs::write(&opt.file, md.as_bytes()).unwrap();

    println!("Converted markdown has been saved in {}.", opt.url);
}
