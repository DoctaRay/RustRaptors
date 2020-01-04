use std::collections::HashMap;
use scraper::{Html, Selector};
extern crate reqwest;
extern crate tokio;

#[tokio::main]
async fn main() -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
    let html = reqwest::get("https://www.foxsports.com/nba/toronto-raptors-team")
        .await?
        .text()
        .await?;
    //println!("{:#?}", html);
    let fragment = Html::parse_fragment(&html);
    let enemy_selector = Selector::parse(r#"div.wisbb_scoresNavigatorChip:nth-child(6) > div:nth-child(3)"#).unwrap();
    let score_selector = Selector::parse(r#"div.wisbb_scoresNavigatorChip:nth-child(6) > div:nth-child(4)"#).unwrap();

    let enemy = fragment.select(&enemy_selector).next().unwrap().text().collect::<Vec<_>>();
    let score = fragment.select(&score_selector).next().unwrap().text().collect::<Vec<_>>();

    print!("Toronto Raptors {} {} took a(n) {}: {}", enemy[1].trim(), enemy[2].trim(), score[1].trim(), score[2].trim());
    Ok(())
}