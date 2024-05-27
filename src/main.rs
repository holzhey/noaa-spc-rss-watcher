use std::{error::Error, fmt::Display};

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
    link: String,
}

impl Display for Warning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut c = self.content.clone();
        c.truncate(150);
        write!(
            f,
            "** {} **\n{}\n{}\n-------------",
            self.title, self.link, c
        )
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
        warnings.push(get_warning(item));
    }
    Ok(warnings)
}

fn get_warning(item: Item) -> Warning {
    let mut desc = "(failed to parse description)";

    let start = item.description.find("<pre>").unwrap_or(0);
    let finish = item.description.find("</pre>").unwrap_or(desc.len());
    dbg!(start, finish);
    if start != 0 && finish != desc.len() {
        desc = &item.description[start..finish];
    }

    Warning {
        title: item.title.to_string(),
        content: desc.to_string(),
        link: item.link,
    }
}

fn get_feed() -> Result<Rss, Box<dyn Error>> {
    let content = reqwest::blocking::get(RSS_ALL)?.text()?;
    let rss: Rss = quick_xml::de::from_str(content.as_str()).unwrap();
    Ok(rss)
}
