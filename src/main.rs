// [] work on error handling
// [] add handling for /newcomments
use clap::{Parser, ValueEnum};
use reqwest::StatusCode;
use scraper::{Html, Selector};

mod utils;

#[derive(ValueEnum, Clone, Debug)]
enum Page {
    News,
    Newest,
    Front,
    // NewComments,
    Ask,
    Show,
    Jobs
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
            Page::Jobs => "jobs"
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
    let title_selector = Selector::parse("span.titleline").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    for element in document.select(&title_selector) {
        let a = element.select(&a_selector).next().unwrap();
        let title = a.inner_html().to_string();
        let href = match a.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found",
        };
        println!("TITLE: {:?} \n", &title);
        println!("LINK: {} \n", &href);
        println!("====================");
    }
    // println!("{}", args.page);
    // println!("{}", args.rows_limit);
}
