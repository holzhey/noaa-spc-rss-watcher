use std::error::Error;

use rss::Channel;
use scraper::{Html, Selector};

const _RSS_SEVERE: &str = "https://www.spc.noaa.gov/products/spcwwrss.xml";
const RSS_ALL: &str = "https://www.spc.noaa.gov/products/spcrss.xml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let channel = get_feed().await?;
    println!("Loaded channel: {}", channel.title());
    for item in channel.items() {
        let title = item.title().unwrap_or("(no title)");
        let desc = item.description().unwrap_or("(nothing)");
        let d = Html::parse_fragment(desc);
        let sel = Selector::parse("pre").unwrap();
        let mut pre = d.select(&sel);

        println!("Item title: {}", title);
        if let Some(n) = pre.next() {
            println!("{}", n.text().next().unwrap());
        }
    }
    Ok(())
}

async fn get_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(RSS_ALL).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
