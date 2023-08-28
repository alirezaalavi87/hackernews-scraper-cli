// [] add pagination
// [] work on error handling
// [] add handling for /newcomments
// [] after finishing this wep scraper version, work on a verion with the hackernews API
use clap::Parser;
use reqwest::StatusCode;
use parse_data::parse_data;

mod args;
mod utils;
mod parse_data;

#[tokio::main]
async fn main() {
    let args = args::Arguments::parse();
    let client = utils::get_client();
    let url = format!("https://news.ycombinator.com/{}", args.path.to_path());
    let res = client.get(url).send().await.unwrap();

    let raw_html = match res.status() {
        StatusCode::OK => res.text().await.unwrap(),
        _ => panic!("Something went wrong in fetching the page"),
    };
    
    parse_data(&raw_html);
}
