use anyhow::Result;
use cxapp::CxApp;
use fltk::enums::{Color, Font};
use fltk_desk::ui::misc::Theme;
mod cxapp;
mod engine;
//mod listbox;
mod mainui;
fn say(s: &str, v: bool) {
    println!("{s}")
}
fn main() -> Result<()> {
    util::say::set_say_cb(say);
    pub const MyDark1: Color = Color::from_rgbi(34);
    let theme: &'static Theme = &Theme {
        bg: Color::Black,
        fg: Color::White,
        hl: Color::Blue,
        popbg: MyDark1,
        frame_color: Color::Dark1,
        font: Font::Helvetica,
        font_size: 14,
        mono_font: Font::Courier,
        mono_font_size: 14,
    };

    let mut app = CxApp::new(theme)?;

    app.run();
    Ok(())
}
