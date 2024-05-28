use std::error::Error;

use app::App;

mod app;
mod tui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init()?;
    App::default().run(&mut terminal)?;
    tui::restore()?;
    Ok(())
}
