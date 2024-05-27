use std::error::Error;

use noaa_spc_rss_parser::get_warnings;

fn main() -> Result<(), Box<dyn Error>> {
    let warnings = get_warnings()?;
    for w in warnings {
        println!("{}", w);
    }
    Ok(())
}
