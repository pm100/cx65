use anyhow::Result;
use cxapp::CxApp;
mod cxapp;
mod engine;
mod listbox;
mod mainui;
fn say(s: &str, v: bool) {
    println!("{s}")
}
fn main() -> Result<()> {
    util::say::set_say_cb(say);
    let mut app = CxApp::new()?;

    app.run();
    Ok(())
}
