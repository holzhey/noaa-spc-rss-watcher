use std::{error::Error, fmt::Display};

use scraper::{Html, Selector};
use serde::Deserialize;

const _RSS_SEVERE: &str = "https://www.spc.noaa.gov/products/spcwwrss.xml";
const RSS_ALL: &str = "https://www.spc.noaa.gov/products/spcrss.xml";

#[derive(Debug, Deserialize)]
struct Rss {
    channel: Channel,
}

#[derive(Debug, Deserialize)]
struct Channel {
    item: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct Item {
    link: String,
    title: String,
    description: String,
}

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

fn main() -> Result<(), Box<dyn Error>> {
    let warnings = get_warnings(get_feed()?)?;
    for w in warnings {
        println!("{}", w);
    }
    Ok(())
}

fn get_warnings(doc: Rss) -> Result<Vec<Warning>, Box<dyn Error>> {
    let mut warnings = Vec::new();
    for item in doc.channel.item {
        let title = item.title;
        let desc = item.description;
        let dom = Html::parse_fragment(&desc);
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

fn get_feed() -> Result<Rss, Box<dyn Error>> {
    let content = reqwest::blocking::get(RSS_ALL)?.text()?;
    let rss: Rss = quick_xml::de::from_str(content.as_str()).unwrap();
    Ok(rss)
}
