use scraper::{Html, Selector};
use colored::*;

pub fn parse_data(raw_html: &String) {
    let document = Html::parse_document(&raw_html);
    let row_selector = Selector::parse("tr.athing").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let rank_selector = Selector::parse("span.rank").unwrap();

    for element in document.select(&row_selector) {
        // select .nth(1) because first <a> contains the upvote link, and second
        // is the posts URL
        let rank = element.select(&rank_selector).next().unwrap().inner_html();
        let a = match element.select(&a_selector).nth(1) {
            Some(a) => a,
            _ => element.select(&a_selector).nth(0).unwrap(),
        };
        let title = a.inner_html().to_string();
        let href = match a.value().attr("href") {
            Some(target_url) => target_url,
            _ => "no url found",
        };

        // println!("{:?}", a);
        println!("{}{}", rank.magenta().dimmed(), &title.bold());
        println!("{} \n", &href.dimmed());
        println!("{}", "==============".purple());
    }
}
