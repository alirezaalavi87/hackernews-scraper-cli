// [] work on error handling
// [] add handling for /newcomments
// [] add pagination
use clap::{Parser, ValueEnum};
use colored::*;
use reqwest::StatusCode;
use scraper::{Html, Selector};

mod utils;

#[derive(ValueEnum, Clone)]
enum Page {
    News,
    Newest,
    Front,
    // NewComments,
    Ask,
    Show,
    Jobs,
}
impl Page {
    pub fn to_path(&self) -> &'static str {
        match self {
            Page::News => "news",
            Page::Newest => "newest",
            Page::Front => "front",
            // Page::NewComments => "newcomments",
            Page::Ask => "ask",
            Page::Show => "show",
            Page::Jobs => "jobs",
        }
    }
}
/// Read news from hackernews
#[derive(Parser)]
struct Arguments {
    /// Which of the pages to fetch. options are:
    #[arg(default_value = "news")]
    #[arg(value_enum)]
    page: Page,
    /// Fetch the first N posts
    // TODO implement rows_limit
    #[arg(default_value = "20")]
    rows_limit: i32,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();
    let client = utils::get_client();
    let url = format!("https://news.ycombinator.com/{}", args.page.to_path());
    let res = client.get(url).send().await.unwrap();

    let raw_html = match res.status() {
        StatusCode::OK => res.text().await.unwrap(),
        _ => panic!("Something went wrong in fetching the page"),
    };

    let document = Html::parse_document(&raw_html);
    let row_selector = Selector::parse("tr.athing").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let rank_selector = Selector::parse("span.rank").unwrap();

    for element in document.select(&row_selector) {
        // select .nth(1) because first <a> contains the upvote link, and second
        // is the posts URL
        let rank = element.select(&rank_selector).next().unwrap().inner_html();
        let a = element.select(&a_selector).nth(1).unwrap();
        let title = a.inner_html().to_string();
        let href = match a.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found",
        };

        println!("{}{}", rank.magenta().dimmed(), &title.bold());
        println!("{} \n", &href.dimmed());
        println!("{}", "==============".purple());
    }
}
