use std::{error::Error, fmt::Display};

use rss::Channel;
use scraper::{Html, Selector};

const _RSS_SEVERE: &str = "https://www.spc.noaa.gov/products/spcwwrss.xml";
const RSS_ALL: &str = "https://www.spc.noaa.gov/products/spcrss.xml";

struct Warning {
    title: String,
    content: String,
}

impl Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = self.content.clone();
        c.truncate(150);
        write!(f, "** {} **\n{}\n-------------", self.title, c)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let warnings = get_warnings(get_feed().await?).await?;
    for w in warnings {
        println!("{}", w);
    }
    Ok(())
}

async fn get_warnings(channel: Channel) -> Result<Vec<Warning>, Box<dyn Error>> {
    let mut warnings = Vec::new();
    for item in channel.items() {
        let title = item.title().unwrap_or("(no title)");
        let desc = item.description().unwrap_or("(nothing)");
        let dom = Html::parse_fragment(desc);
        let sel = Selector::parse("pre")?;
        let mut pre = dom.select(&sel);

        if let Some(n) = pre.next() {
            warnings.push(Warning {
                title: title.to_string(),
                content: n.text().next().unwrap().to_string(),
            })
        }
    }

    Ok(warnings)
}

async fn get_feed() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(RSS_ALL).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
