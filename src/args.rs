use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone)]
pub enum Path {
    News,
    Newest,
    Front,
    // NewComments,
    Ask,
    Show,
    Jobs,
}
impl Path {
    pub fn to_path(&self) -> &'static str {
        match self {
            Path::News => "news",
            Path::Newest => "newest",
            Path::Front => "front",
            // Page::NewComments => "newcomments",
            Path::Ask => "ask",
            Path::Show => "show",
            Path::Jobs => "jobs",
        }
    }
}
/// Read news from hackernews
#[derive(Parser)]
pub struct Arguments {
    /// Which of the pages to fetch. options are:
    ///
    #[arg(default_value = "news")]
    #[arg(value_enum)]
    pub path: Path,
    /// Pagination
    #[arg(default_value = "20")]
    page: i32,
}
